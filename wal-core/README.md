# Wal-core

Wal-core is the most crucial part of a wal project. It enables the programmer to:
 - Define components
 - Define routing rules
 - Create event handlers for browser events
 - Define the stucture of the page using VDOM

To create components a Component trait needs to be implemented on the type.
To define routing rules a RouterBuider needs to be used.
Defining handlers for a browser events is done by simply creating closures.
VDOM can be created just like every normal struct or it can be defined using rsx macro from wal-rsx.