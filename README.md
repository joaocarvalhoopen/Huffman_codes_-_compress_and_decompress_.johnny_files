# Huffman codes - compress and decompress .johnny files
Use with small or larger files, text or binary.


## Description
This program is a simple application of Huffman codes to do compression (encode) and decompression (decode) of a text or binary file (message as the message byte symbols). Because the one making the program gives it's extension name, the extension of the compressed files are **.johnny** . At the moment can only compress one file in each execution. It print's the Hufffman code tree and the map_encoding table. Developed on Linux, but in principal can be compiled also to Windows or Mac. <br> 
<br>
See the following link for the beautiful details and a deeper understanding of the Huffman codes.<br> 

* Huffman Codes: An Information Theory Perspective <br>
  [https://www.youtube.com/watch?v=B3y0RsVCyrw](https://www.youtube.com/watch?v=B3y0RsVCyrw)


## Compilation 
Place the files of this project in a directory called ```huffman_codes``` and make the following command. <br>

```
cargo build --release
```

The executable will be in ```huffman_codes/target/release/huffman_codes```


## Usage: 

* **to compress a text or binary file** do: <br>
```
  ./huffman_codes compress input_text.txt
```

* **to decompress a compressed text or binary file** do: <br>
```
  ./huffman_codes decompress output_text.txt.johnny
```


## Timings

* [ in **SSD**, **std HasMap**] <br>
  **compress:**    **3.4 MB** -> **2.4 MB** executable in to **.johnny** in **0.211 s** <br>
  **decompress:**  2.4 MB -> 3.4 MB .johnny in to executable in **0.521 s**

* [ in **RAM /dev/shm/**, **std HashMap** ] <br>
  **compress:**    **600 MB** -> 600MB + 2570 Bytes mp4 video in to .johnny in **33.825 s** <br>
  **decompress:**  600MB + 2570 Bytes -> 600 MB .johnny in to mp4 video in **1m 44.326 s** 
                                                                             
* [ in **RAM /dev/shm/**, **fast HashBrown HashMap** ] <br>
  **compress:**    **600 MB** -> 600MB + 2570 Bytes mp4 video in to .johnny in **23.863 s** <br>
  **decompress:**  600MB + 2570 Bytes -> 600 MB .johnny in to mp4 video in **51.689 s** <br>
  Note: This is the current code version.


## Algorithm

1. First we will read the parameters and decide if we will call the function compress (encode) or the function decompress (decode).
<br>

**Function compress:** <br>
1. Read all of the input file in binary buffer. So we have a one byte representation of each symbol, this step will make the problem.
2. Determine the frequency of the symbols (different bytes) in the input buffer.
3. By using a priority queue and the Huffman coding tree find the best coding for each symbol of the message. Create a table for the code. This table inverted will also have to be known in the decoding phase.
4. Write the table to the beginning of byte buffer and 16 bit header, with the start of the data in the buffer_out.
5. With the new dictionary, encode the message in bytes to a byte buffer.
6. Write the first 8 byte with an usize 64 bit's representing the number of bytes or total symbols in the original file of the message.
7. Write the final compressed byte buffer to file .johnny .
<br>

**Function decompress:** <br>
1. Read the file from disk into a byte buffer in binary representation.
2. Extract the symbols coding table to an internal representation. That is, the one with the Huffman coding inverted for decoding.
3. Read the 16 bit header with the index (of the byte) of the start of the data in the .johnny file. Read the second header with the number of original symbols, or we could say original bytes. Apply the decoding table to the coded message bytes, buffer_in, and decode or decompress it into a binary buffer_out.
4. Write to the output file of the decoded binary or text data. 
<br>


## References

* **Huffman Codes**: An Information Theory Perspective <br>
  [https://www.youtube.com/watch?v=B3y0RsVCyrw](https://www.youtube.com/watch?v=B3y0RsVCyrw)

* **hashbrown** - Fast drop in replacement for STD HashMap, a Rust port of Google's high-performance SwissTable hash map. <br>
  [https://github.com/Amanieu/hashbrown](https://github.com/Amanieu/hashbrown)


## License
MIT Open Source


## Have fun!
Best regards, <br>
João Nuno Carvalho
