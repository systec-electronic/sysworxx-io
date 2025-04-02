# Running CODESYS runtime on sysWORXX devices

## Installation on device

### AM62x

- Install packages for "CODESYS Virtual Control for Linux ARM64 SL" in IDE
- Tools - Control SL ausrollen
  - Kommunikation: SSH Login on the device
  - Bereitstellung:
    - Produkt: CODESYS Virtual Control for Linux SL
    - Version: 4.13.0 (arm64) - or newer
    - Click Installieren
  - Operation:
    - In window VPLCs click on plus symbol, select image and give any name
    - Click in window VPLCs on the image - settings window opens to the right
    - Enter following:
      - Mounts: `/var/opt/codesysvcontrol/instances/CODESYS/conf/codesyscontrol:/conf/codesyscontrol/, /var/opt/codesysvcontrol/instances/CODESYS/data/codesyscontrol:/data/codesyscontrol/,  /var/run/codesysextension/extfuncs/: /var/run/codesysextension/extfuncs/`
      - Ports: `11740:11740`
