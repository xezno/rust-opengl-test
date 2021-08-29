# Rust OpenGL Test

Terrible Rust rendering thing.

## Screenshots

![Monkeys](https://cdn.discordapp.com/attachments/839155256964284459/881512752491941938/unknown.png)

## Things to note

### 1. Lots of unsafe

I don't know much Rust but I know enough about programming that writing the word `unsafe` means you can do bad stuff.
In this situation though, I have to use it in order to access the OpenGL api, I think that doing so is generally better than
using some bozo's graphics abstraction library that I don't understand and will probably be abandoned in 3 months... so
I've opted to just write bare ogl code and not give a shit.

### 2. This isn't for you

I'm writing this as a learning project. It won't become a game. It's not an "engine". It's not something you can use at all
unless you really want to compile it and look at some funny 3d models for 30 seconds, then realize that the 5 minute download
and compile process was a waste of your time.

### 3. I don't know what I'm doing

But does anyone?
