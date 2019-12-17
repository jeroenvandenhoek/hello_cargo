import wasm from './pkg/web_sys_canvas_hello_world.js'
import { start } from './pkg/web_sys_canvas_hello_world.js' 
wasm()
	.then(m=>{
		start()
	})
