# Flucht (german for escape)

It's a first small looking game, with the target of getting the 2x2 stone on the other side of the "table". You're only allowed to move the stones.
This project solves it.

To see the results, look at the terminal or open `solution.html` after it finished.

Credits to the [Illuseum Berlin](https://www.illuseum-berlin.de/)

Run `cargo run --release` to start, and please use [wsl2](https://docs.microsoft.com/en-us/windows/wsl/install) on windows for best performance

Example `solution.html`:

<h1 style="text-align: center; font-family: Arial, Helvetica, sans-serif">Solution</h1>
<div style="display: flex; flex-wrap: wrap; justify-content: space-around; gap: 20px;">

<pre>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="background-color: rgb(125, 78, 87);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(236, 78, 32);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>      
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>      <mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
      <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
   <mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
      <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>   
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>      
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
      <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
      <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>      <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>      
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>      
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark>   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>   
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark>   <mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark>   <mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(66, 72, 116);">   </mark>   <mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
   <mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>      
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>      
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>   
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
      <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
      <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
   <mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>      <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>   
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark>      
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>      
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>   
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
<mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
<mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark>   <mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

<pre>
<mark style="color: rgb(255, 255, 255); background-color: rgb(125, 78, 87)"> 5 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(66, 72, 116)"> 4 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(100, 149, 5)"> 2 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(236, 78, 32)"> 1 </mark>
<mark style="background-color: rgb(125, 78, 87);">   </mark><mark style="background-color: rgb(66, 72, 116);">   </mark><mark style="background-color: rgb(100, 149, 5);">   </mark><mark style="background-color: rgb(236, 78, 32);">   </mark>
<mark style="color: rgb(0, 0, 0); background-color: rgb(224, 202, 60)"> 9 </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(147, 225, 216)"> 6 </mark><mark style="color: rgb(255, 255, 255); background-color: rgb(125, 120, 90)"> 3 </mark><mark style="background-color: rgb(125, 120, 90);">   </mark>
   <mark style="color: rgb(255, 255, 255); background-color: rgb(1, 111, 185)"> 0 </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(238, 99, 102)"> 8 </mark>
   <mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="background-color: rgb(1, 111, 185);">   </mark><mark style="color: rgb(0, 0, 0); background-color: rgb(63, 239, 45)"> 7 </mark>
</pre>

</div>