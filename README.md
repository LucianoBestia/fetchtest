# fetchtest

*OBSOLETE: finally we have async/await in Rust. Look my project fetchtest2 2020-01-05*

I am trying to use the fetch api in Rust Wasm.  
It is such a frequent functionality for frontend applications.  
The Promises and Futures are very hard to understand here. I had a lot of problems.  
The documentation is scarse and incomplete. The working examples are not working or too complex.  
I need the simplest possible fetch: take a simple webpage somewhere and show it on the screen.  
I don't really care about performance here.  

## javascript

First I will do this simple example in javascript.  
That was simple. It works. The coding style of Promises is not really nice, but is not terrible.  

## js async + await

New javascript versions have async+await. That looks really good readable code. Finally.  

## rust

Then I try to use wasm_bindgen promise. This is a crude representations in rust of the javascript object.  
But as always with typeless languages like javascript you never know what is really inside that promise.  
Rust is a typefull language and we must be aware of the type in any moment.  
Usig a reference to a Closure is terrible. It makes the code look very unreadable.  
So after years (decades) of learning all the beauty of functions, methods, parameters, function calls and returns, now we have something that does practically the same thing, but the code is completely unreadable for a normal human. We have gone really far from simple and logical.  
Functions are now suddenly only variables, but they can contain Closures. The return value is always the same "promise object", but inside of that is something else we don't really know. Maybe it is written down and explained somewhere in some obscure documentation. But I didn't find it.  
From javascript we get always the generic JsValue that can be anything at all...  
Pretty terrible situation.  
It can work like that. Everything can work, that is not the point. Where and when have we lost the beauty of elegant coding and now we are making something like a ball of strings for the cat?  
<https://fundamentalsofcode.com/javascript-fetch-api-and-using-async-await/>  

## futures and promises

Rust has futures. JavaScript has promises. They are same-same, but different.  
I don't really understand why I should jump from futures to promises and back?  
Why not just use promises? What is so much better using futures?  

## Better code

After all this ranting from an old man and spending a lot of time and effort I have finally something I like.  
I used the fetch example that converts from promises to futures. From that I created a generic
function that encapsulates all the promise/future chain. At the end of this chain we want to do something
useful. So I must send a reference to a function as a parameter. It works! And I like it because the complicated stuff is 'hidden' in a single function. The function that really executes something useful is just a simple function as usual.

## rust async+await

It is still in the working. Very promising, but it takes forever to get released in stable...  

## build and run

Type and run this in the project folder  
`cargo make dev`  
To install cargo-make  
`cargo install --force cargo-make`
