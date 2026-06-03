# mini-projects-rust
Mini projects built with rust

## Projects
- Simplegrep
- Webserver

### Webserver
Created this webserver by following it here -> [Rust book webserver](https://doc.rust-lang.org/book/ch21-00-final-project-a-web-server.html). I didn't exactly follow it since. I already know what goes into a webserver. I just skim through this section and see what the requirements are and then build it on my own.



**Current implementation**: 
I believe other than the threadpool, everything is very straight-forward simple TCP binding TCPListener (builtin std library), then handle the upcoming request. Take each request (stream) and then offload it to a new thread. Now if we just keep creating threads the whole operation will become super expensive.<br />
So what I did was created a threadpool. As you'll see in my code my implementaion is a bit different from the book, as I was following the requirement rather than exactly what was in the book. <br />
And along with that I also realized my mistake at the end. My implementation of threadpool included n channels for n no. of threads in the threadpool and a simple vector for performing round-robin algo to assign the stream to a thread one-by-one.<br />
Well it is in a way less compilcated as you don't have to deal with the thought of race conditions and all. But anyways the books method is more efficient as they only have one channel through which receiver is shared among multiple threads using Arc<Mutex> now you will think why is that since mpsc (muliple producer single consumer) can only have one receiver. Well here also in the books implementation we only have 1 receiver, it's just that we are using mutex through which we can basically lock the access per thread. 
