use neon::prelude::*;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};
#[macro_use]
extern crate lazy_static;

#[path = "../../Rust/sysworxx_io.rs"]
pub mod sysworxx_io;
use sysworxx_io::*;

#[derive(Default, Debug)]
struct Instance {
    input_interrupt_handlers: HashMap<u8, HashMap<usize, Root<JsFunction>>>,
    uid: usize,
    neon_channel: Option<neon::event::Channel>,
}

lazy_static! {
    static ref INSTANCE: Arc<Mutex<Instance>> = Arc::new(Mutex::new(Instance::default()));
}

fn output_set(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let value = cx.argument::<JsBoolean>(1)?.value(&mut cx) as bool;
    let ret = unsafe { IoSetOutput(channel, IoBool::from(value)) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn input_get(mut cx: FunctionContext) -> JsResult<JsBoolean> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mut state: IoBool = IoBool::False;
    let ret = unsafe { IoGetInput(channel, &mut state) };

    match ret {
        IoResult::Success => Ok(cx.boolean(*state)),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn get_hw_info(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let file_path = cx.argument::<JsString>(0)?.value(&mut cx) as String;
    let ret = unsafe { IoGetJson(file_path.as_ptr() as *const std::os::raw::c_char) };
    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn register_input_interrupt(mut cx: FunctionContext) -> JsResult<JsString> {
    //return uid from Instance -> remember in node
    let interface_channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let callback_function = cx.argument::<JsFunction>(1)?.root(&mut cx);
    let interrupt_trigger = match cx.argument::<JsNumber>(2)?.value(&mut cx) as u32 {
        0 => IoInputTrigger::None,
        1 => IoInputTrigger::RisingEdge,
        2 => IoInputTrigger::FallingEdge,
        3 => IoInputTrigger::BothEdge,
        _ => IoInputTrigger::None,
    };
    let instance = INSTANCE.lock();

    match instance {
        Ok(mut instance) => {
            let current_uid = instance.uid;
            instance.uid += 1;

            instance
                .input_interrupt_handlers
                .entry(interface_channel)
                .or_default()
                .insert(current_uid, callback_function);

            let ret = unsafe {
                IoRegisterInputCallback(interface_channel, Some(input_callback), interrupt_trigger)
            };

            match ret {
                IoResult::Success => Ok(cx.string(current_uid.to_string())),
                _ => cx.throw_error(format!("{:?}", ret)),
            }
        }
        Err(_) => cx.throw_error(format!("Error while reading sysWORXX-io instance!")),
    }
}

fn unregister_input_interrupt(mut cx: FunctionContext) -> JsResult<JsString> {
    let input_channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let callback_uid = cx
        .argument::<JsString>(1)?
        .value(&mut cx)
        .parse::<usize>()
        .unwrap();
    let instance = INSTANCE.lock();
    match instance {
        Ok(mut instance) => {
            let callback_function = instance
                .input_interrupt_handlers
                .entry(input_channel)
                .or_default()
                .remove(&callback_uid);
            match callback_function {
                Some(callback_function) => callback_function.drop(&mut cx),
                None => {}
            }
            Ok(cx.string("good"))
        }
        Err(_) => cx.throw_error(format!("Error while reading sysWORXX-io instance!")),
    }
}

extern "C" fn input_callback(channel: u8, value: u8) -> () {
    let instance = INSTANCE.lock();
    match instance {
        Ok(instance) => match &instance.neon_channel {
            Some(neon_channel) => {
                neon_channel.send(move |mut cx| {
                    let mut instance = INSTANCE.lock().unwrap();
                    let hash_map = instance
                        .input_interrupt_handlers
                        .entry(channel)
                        .or_default();
                    for val in hash_map.values() {
                        let callback = val.clone(&mut cx).into_inner(&mut cx);
                        let this = cx.undefined();
                        let args = vec![cx.number(channel).upcast(), cx.number(value).upcast()];
                        callback.call(&mut cx, this, args)?;
                    }
                    Ok(())
                });
            }
            None => {}
        },
        Err(_) => {}
    }
}

fn set_err_led(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsBoolean>(0)?.value(&mut cx);
    let ret = unsafe { IoSetErrLed(IoBool::from(value)) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn set_run_led(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let value = cx.argument::<JsBoolean>(0)?.value(&mut cx);
    let ret = unsafe { IoSetRunLed(IoBool::from(value)) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn analog_input_get(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mut value: u16 = 0;
    let ret = unsafe { IoAdcGetValue(channel, &mut value) };

    match ret {
        IoResult::Success => Ok(cx.number(value)),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn analog_mode_set(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mode = cx.argument::<JsNumber>(1)?.value(&mut cx) as u8;
    let ret = unsafe { IoAdcSetMode(channel, IoAnalogMode::from(mode)) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn tmp_input_get(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mut value: i32 = 0;
    let ret = unsafe { IoTmpGetValue(channel, &mut value) };

    match ret {
        IoResult::Success => Ok(cx.number(value)),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn tmp_set_mode(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mode = cx.argument::<JsNumber>(1)?.value(&mut cx) as u8;
    let temp_type = cx.argument::<JsNumber>(2)?.value(&mut cx) as u8;

    let ret = unsafe {
        IoTmpSetMode(
            channel,
            IoTmpMode::from(mode),
            IoTmpSensorType::from(temp_type),
        )
    };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn counter_enable(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let state = cx.argument::<JsBoolean>(1)?.value(&mut cx);
    let ret = unsafe { IoCntEnable(channel, IoBool::from(state)) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn counter_setup(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mode = cx.argument::<JsNumber>(1)?.value(&mut cx) as u8;
    let trigger = cx.argument::<JsNumber>(2)?.value(&mut cx) as u8;
    let direction = cx.argument::<JsNumber>(3)?.value(&mut cx) as u8;
    let ret = unsafe {
        IoCntSetup(
            channel,
            IoCntMode::from(mode),
            IoCntTrigger::from(trigger),
            IoCntDirection::from(direction),
        )
    };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn counter_preload(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let preload = cx.argument::<JsNumber>(1)?.value(&mut cx) as i32;
    let ret = unsafe { IoCntSetPreload(channel, preload) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn counter_get(mut cx: FunctionContext) -> JsResult<JsNumber> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let mut value: i32 = 0;
    let ret = unsafe { IoCntGetValue(channel, &mut value) };

    match ret {
        IoResult::Success => Ok(cx.number(value)),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn analog_output_set(mut cx: FunctionContext) -> JsResult<JsUndefined> {
    let channel = cx.argument::<JsNumber>(0)?.value(&mut cx) as u8;
    let value = cx.argument::<JsNumber>(1)?.value(&mut cx) as u16;
    let ret = unsafe { IoDacSetValue(channel, value) };

    match ret {
        IoResult::Success => Ok(cx.undefined()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

fn init_sysworxx_io(cx: &mut ModuleContext) -> NeonResult<()> {
    let ret = unsafe { IoInit() };
    match ret {
        IoResult::Success => Ok(()),
        _ => cx.throw_error(format!("{:?}", ret)),
    }
}

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    init_sysworxx_io(&mut cx)?;
    let mut instance = INSTANCE.lock().unwrap();
    instance.neon_channel = Some(cx.channel());

    cx.export_function("output_set", output_set)?;
    cx.export_function("input_get", input_get)?;
    cx.export_function("register_input_interrupt", register_input_interrupt)?;
    cx.export_function("unregister_input_interrupt", unregister_input_interrupt)?;
    cx.export_function("get_hw_info", get_hw_info)?;
    cx.export_function("analog_input_get", analog_input_get)?;
    cx.export_function("analog_mode_set", analog_mode_set)?;
    cx.export_function("tmp_input_get", tmp_input_get)?;
    cx.export_function("tmp_set_mode", tmp_set_mode)?;
    cx.export_function("counter_enable", counter_enable)?;
    cx.export_function("counter_setup", counter_setup)?;
    cx.export_function("counter_preload", counter_preload)?;
    cx.export_function("counter_get", counter_get)?;
    cx.export_function("analog_output_set", analog_output_set)?;
    cx.export_function("set_err_led", set_err_led)?;
    cx.export_function("set_run_led", set_run_led)?;

    Ok(())
}
