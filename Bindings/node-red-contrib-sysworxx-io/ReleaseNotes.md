# v2.0.1 (2023-04-04)

Fix issue with using user defined voltage or current option with analog in.

# v2.0.0 (2023-03-28)

This release contains breaking changes for **every** node. This means, if
you use nodes with any version older than v2.0.0, you have to delete the old
nodes from the flow in Node-RED and then add and configure them again.

Changes:

- Shorten every node name, e.g. "Digital Ouptut" to "Digital Out"
- Add node "Run Switch"
- Add the feature to change the send or expected payload
- Add the feature to change the send or expected topic
- Add and change used node icons
- Change default payload on digital nodes from bool(true, false) to number (1, 0)
- Change node status indicator for nodes to reflect the actual behaviour

# v1.0.0 (2023-02-23)

- Initial version
