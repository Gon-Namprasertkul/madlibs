<h1 href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" class="glow">madlibs</h1>

a basic madlibs game written in rust

<h1 href="https://www.youtube.com/watch?v=dQw4w9WgXcQ" class="glow">installation</h1>

```
git clone https://github.com/ForesterBlox/madlibs
cd mablibs
cargo build --release
```

move target/release/madlibs to a known location and enjoy!





<style>
    body {
    color: white;
    background-color: black;
    font-family: Trebuchet MS;
    font-size: 35px;
    scroll-behavior: smooth;
    height: 100%;
 }
 .glow {
    font-size: 20px;
    color: #fff;
    text-align: center;
    -webkit-animation: glow 1s ease-in-out infinite alternate;
    -moz-animation: glow 1s ease-in-out infinite alternate;
    animation: glow 1s ease-in-out infinite alternate;
  }
  
  @keyframes glow {
    from {
      text-shadow: 0 0 10px #fff, 0 0 20px #fff, 0 0 30px #2550a1, 0 0 40px #2550a1, 0 0 50px #2550a1, 0 0 60px #2550a1, 0 0 70px #2550a1;
    }
    to {
      text-shadow: 0 0 20px #fff, 0 0 30px #2550a1, 0 0 40px #2550a1, 0 0 50px #2550a1, 0 0 60px #2550a1, 0 0 70px #ff4da6, 0 0 80px #2550a1;
    }
  }
</style>
