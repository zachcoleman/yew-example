# yew-example
A small example application using Rust, WASM, and Yew meant as a reference and scaffold for other projects.

This application just makes a grid of clickable squares that can then be reset. The code has an example of 2 levels of hierarchy:

App -> Grid -> Cell

Trying to adhere to best practices: the information from parent to child components flow through `props` and `callbacks` send messages from children to parent components. 

https://user-images.githubusercontent.com/42484306/183918788-bb0b85dd-4d33-48dc-bfec-834152b62de6.mov
