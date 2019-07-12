# fetchtest
I am trying to use the fetch api in Rust Wasm, but I cannot make it work.  
I am desperate. It is such an easy and frequent functionality for frontend applications.  
The Promises and Futures are very hard to understand here.  
I need the simplest possible fetch: take a simple webpage somewhere and show it on the screen.  
I don't really care about performance here.  
## javascript
First I will do this simple example in javascript.  
That was simple. It works. The coding style ofPromises is not really nice, but is not terrible.  
## js async + await
New javascript have async+await. That looks really readable code. Finally.  
## rust
Then I try to use wasm_bindgen promise. This is a crude representations in rust of a javascript object.  
But as always with typeless languages like javascript you never know what is really inside that promise.  
Rust is a typefull language and we must be aware of the type in any moment.  
Usig a reference to a Closure is terrible. It looks very unreadable.  
So after years (decades) learning all the beauty of functions, methods, parameters, function calls and returns, now we have something that does practically the same thing, but the code is completely unreadable.  
Functions are now suddenly variables, but they contain Closures. The return value is always the same "promise", but inside of that is something else we don't really know. Maybe it is written in some obscure documentation. From javascript we get always the generic JsValuethat can be anything at all...  
Pretty terrible situation.  
It can work that. Everything can work, that is not the point. Where and when have we lost the beauty of coding and now we make something like a ball of strings for the cat?  
https://fundamentalsofcode.com/javascript-fetch-api-and-using-async-await/  
##rust async+await
It is still in the working. Very promising, but it takes forever...  
## build and run
Type and run this in the project folder  
`cargo make dev`
To install cargo-make  
`cargo install --force cargo-make`





