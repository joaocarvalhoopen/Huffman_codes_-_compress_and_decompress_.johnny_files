/******************************************************************************
* Program name: huffman_codes                                                 *
* Author: João Nuno Carvalho                                                  *
* Date: 2021.08.01                                                            *
*                                                                             *
* Description: This program is a simple application of Huffman codes to do    *
*              compression (encode) and decompression (decode) of a text      *
*              or binary file (message as the message byte symbols).          * 
*              Because the one doing the program gives it's extension name,   *
*              the extension of the compressed files are .johnny .            *
*              At the moment can only compress one file in each execution.    *
*                                                                             *
* See the following link for the beautiful details and a deeper understanding *
* of the Huffman codes.                                                       *
*                                                                             *
* Huffman Codes: An Information Theory Perspective                            *
* https://www.youtube.com/watch?v=B3y0RsVCyrw                                 *
*                                                                             *
* Usage:                                                                      *
*                                                                             *
* to compress a text or binary file do:                                       *
* huffman_codes compress input_text.txt                                       *
*                                                                             *
* to decompress a compressed text or binary file do:                          *
* huffman_codes decompress output_text.txt                                    *
*                                                                             *
*                                                                             *
* Algorithm:                                                                  *
* 1. First we will read the parameters, validate them and decide if we will   *
*    call the function compress (encode) or the function decompress (decode). *
*                                                                             *
*                                                                             *
* Function compress:                                                          *
* 1. Read all of the input file in binary buffer. So we have a one byte       *
*    representation of each symbol, this step will make the problem.          *
* 2. Determine the frequency of the symbols (different bytes) in the input    *
*    buffer.                                                                  *
* 3. By using a priority queue and the Huffman coding tree find the best      *    
*    coding for each symbol of the message. Create a table for the code.      *
*    This table inverted will also have to be known in the decoding phase.    *
* 4. Write the table to the beginning of byte buffer and 16 bit header,       *
*    with the start of the data in the buffer_out.                            *
* 5. With the new dictionary, encode the message in bytes to a byte buffer.   *
* 6. Write the first 8 byte with an usize 64 bit's representing the number    *
*    of bytes or total symbols in the original file of the message.           *
* 7. Write the final compressed byte buffer to file .johnny .                 *
*                                                                             *
*                                                                             *
* Function decompress:                                                        *
* 1. Read the file from disk into a byte buffer in binary representation.     *
* 2. Extract the symbols coding table to an internal representation. That is, *
*    the one with the Huffman coding inverted for decoding.                   *
* 3. Read the 16 bit header with the index (of the byte) of the start of      *
*    the data in the .johnny file. Read the second header with the number     *
*    of original symbols, or we could say original bytes.                     *
*    Apply the decoding table to the coded message bytes, buffer_in, and      *
*    decode or decompress it into a binary buffer_out.                        *
* 4. Write to the output file of the decoded binary or text data.             *
*                                                                             *
*                                                                             *
* License: MIT Open Source                                                    *
*                                                                             *
* Time:                                                                       *
*                                                                             *
*   [ in SSD, std HasMap]                                                     *
*   compress    3.4 MB -> 2.4 MB executable in to .johnny in 0.211 s          *
*   decompress  2.4 MB -> 3.4 MB .johnny in to executable in 0.521 s          *
*                                                                             *
*   [ in RAM /dev/shm/, std HashMap ]                                         *
*   compress    600 MB -> 600MB + 2570 Bytes mp4 in to .johnny in 33.825 s    *
*   decompress  600MB + 2570 Bytes -> 600 MB .johnny in to mp4 in 1m 44.326 s *
*                                                                             *
*   [ in RAM /dev/shm/, fast HashBrown HashMap ]                              *
*   compress    600 MB -> 600MB + 2570 Bytes mp4 in to .johnny in 23.863 s    *
*   decompress  600MB + 2570 Bytes -> 600 MB .johnny in to mp4 in 51.689 s    *
*                                                                             *
*                                                                             *
******************************************************************************/

//use std::collections::HashMap;

// The fastest HashMap for Rust. HashBrown a drop in replacement for std HashMap.
use hashbrown::HashMap;

use std::env;
use std::io::{Read, Write};
use std::process;
use std::path::Path;
use std::fs::File;
// use std::fs::Metadata;
// use std::io::Read;
use std::io::BufReader;  // Faster :-D
use std::io::BufWriter;  // Faster :-D
use std::ffi::OsStr;
// use priority_queue::PriorityQueue;      // for Huffman code algorithm.
// use priority_queue_rs::PriorityQueue;   // for Huffman code algorithm.

/// Usage: "huffman_codes [compress|decompress] filename"
static USAGE: &str = "   Usage: \"huffman_codes [compress|decompress] filename";

fn main() {
    println!("***********************************************************");
    println!("** Huffman codes - compress and decompress .johnny files **");
    println!("***********************************************************");
    let args: Vec<String> = env::args().collect();
    let cfg = Config::new(& args);
    match cfg.action {
        Action::Compress   => compress( & cfg ),
        Action::Decompress => decompress( & cfg ),
    }
    println!("...ended processing the file.");
}

#[derive(Debug)]
#[derive(PartialEq)]
enum Action {
    Compress,
    Decompress
}

/// Configuration structure to parse the command line options.  
#[derive(Debug)]
struct Config {
    action: Action,
    filename: String,
}

impl Config {
    /// Constructor - Is were the parsing is made.
    /// It exists if an error occurs.
    fn new(args: &[String]) -> Config {
        if args.len() != 3 {
            println!(" Invalid or insufficient parameters...");
            println!("{}", USAGE);
            process::exit(0)
        }
        // casting your String into an &str (a string slice)
        let action = match &( args[1].to_ascii_uppercase() )[..] {
            "COMPRESS"   => Action::Compress,
            "DECOMPRESS" => Action::Decompress,  
            _ => {    
                println!(" Invalid compress or decompress action ex: huffman_codes compress  ...");
                println!("{}", USAGE);
                process::exit(0)
            } 
        };

        let filename: String = args[2].to_string();
        // Validate if filename exists.
        let file_path = Path::new( &filename );
        if !( file_path.exists() ) {
            println!(" Invalid or not existing filename '{}'", filename);
            println!("{}", USAGE);
            process::exit(0)
        }

        if action == Action::Decompress { 
            // If is Action.decompress, validates if it ends with a .johnny extension :-D hehehehe!                      
            let flag_error_in_extension = match file_path.extension().and_then(OsStr::to_str) {
                    Some(s) => if s.to_lowercase() == ("johnny") {
                                        false
                                    } else {
                                        true
                                    },
                    None         => true,  
                };
            
            if flag_error_in_extension {
                println!(" Can't decompress a file without the extension .johnny ... '{}'", filename);
                println!("{}", USAGE);
                process::exit(0)
            }
        }
        
        Config { action, filename }
    }
}

fn compress(cfg: & Config) {
    println!("...start compressing file {}", cfg.filename);

    // 1. Read all of the input file in binary buffer. So we have a one byte
    //    representation of each symbol, this step will make the problem.
    let buffer_in: Vec<u8> = get_file_as_byte_vec( &cfg.filename );
    let mut buffer_out: Vec<u8> = Vec::new();

    if buffer_in.len() <= 2 {
        buffer_out = buffer_in;
    } else {

        // 2. Determine the frequency of the symbols (different bytes) in the input buffer.
        let mut map_table = MappingTable::new();
        map_table.get_buffer_byte_symbols_freq(& buffer_in);
            
        // 3. By using a priority queue and the Huffman coding tree find the best    
        //    coding for each symbol of the message. Create a table for the code.
        //    This table inverted will also have to be known in the decoding phase.
        map_table.generate_huffman_code();

        // 4. Write the table to the beginning of byte buffer and the 16 bit heading,
        //    with the start of the data.
        map_table.write_mapping_table_to_byte_buffer(& mut buffer_out);

        // 5. With the new dictionary, encode the message in bytes to a byte buffer.
        // 6. Write the first 8 byte with an usize 64 bit's representing the number
        //    of bytes or total symbols in the original file of the message. 
        map_table.encode_the_data(& buffer_in, & mut buffer_out);

    }    

    // 7. Write the final compressed byte buffer to file .johnny,
    let compressed_filename: String = cfg.filename.clone() + ".johnny"; 
    write_byte_vec_to_file(& compressed_filename, &buffer_out);
    
    println!("...finish writing compressed file {}", compressed_filename);
}

fn decompress(cfg: & Config) {
    println!("...start decompressing file {}", cfg.filename);

    // 1. Read the file from disk into a byte buffer in binary representation.
    let buffer_in: Vec<u8> = get_file_as_byte_vec( &cfg.filename );
    let mut buffer_out: Vec<u8> = Vec::new();

    if buffer_in.len() <= 2 {
        buffer_out = buffer_in;
    } else {

        // 2. Extract the symbols coding table to an internal representation. That is
        //    the one with the Huffman coding inverted for decoding. 
        let mut map_table = MappingTable::new();
        let header_2_start = map_table.read_mapping_table_from_byte_buffer(& buffer_in);

        // 3. Read the 16 bit header with the index (of the byte) of the start of
        //    the data in the .johnny file. Read the second header with the number
        //    of original symbols, or we could say original bytes. 
        //    Apply the decoding table to the coded message bytes, buffer_in, and decode or
        //    decompress it into a binary buffer_out. 
        map_table.decode_the_data(& buffer_in, & mut buffer_out, header_2_start);

    }

    // 4. Write to the output file of the decoded binary or text data.
    let string_tmp = cfg.filename.clone();
    let (decompressed_filename, _): (&str, &str) = string_tmp.split_at(string_tmp.len() - ".johnny".len()); 
    write_byte_vec_to_file(& decompressed_filename.to_string(), &buffer_out);

    println!("...finish writing decompressed file {}", decompressed_filename);
}

// Read binary file as byte vector (u8).
// From: https://www.reddit.com/r/rust/comments/dekpl5/how_to_read_binary_data_from_a_file_into_a_vecu8/
fn get_file_as_byte_vec(filename: &String) -> Vec<u8> {
    let f = File::open(&filename).expect("file not found.");
    let metadata = std::fs::metadata(&filename).expect("unable to read metadata.");
    let mut buffer = vec![0; metadata.len() as usize];
    let mut buf_reader = BufReader::new(f);
    buf_reader.read(&mut buffer).expect("...buffer overflow.");
    // Note: The file closes automatically when it gets out of scope.

    buffer
}

// Write binary byte vector (u8) to a file.
fn write_byte_vec_to_file(filename: &String, buffer: & Vec<u8>) {
    let mut f = File::create(&filename).expect("no file found");
    let mut buf_writer = BufWriter::new(& mut f);
    buf_writer.write_all(&buffer).expect("...error while writing file!");
    buf_writer.flush().expect("error while writing file!");
}
enum Elem {
    Node(NodeType),
    Leaf(LeafType),
} 

struct NodeType {
    total_count: usize,
    left:        Box<Elem>,
    right:       Box<Elem>,
}

struct LeafType {
    pos: u8,
    count: usize,
}

struct MappingTable {
    vec_elem_count:  Vec<Elem>,        // Vec<(u8, usize)>,
    print_text_char: bool,
    map_encoding:    HashMap< u8, String >,
    map_decoding:    HashMap< String, u8 >,
}

impl MappingTable {

    fn new() -> MappingTable {
        MappingTable {
            vec_elem_count:  Vec::new(),
            print_text_char: true,
            map_encoding:    HashMap::new(),
            map_decoding:    HashMap::new(),
        }
    }


    ///******************
    ///* Compress methods
    ///******************

    /// 2. Determine the frequency of the symbols (different bytes) in the input buffer.
    fn get_buffer_byte_symbols_freq(& mut self, buffer_in: & Vec<u8>) {
        let mut map_freq: [usize; 256] = [0; 256];
        for &elem in buffer_in {
            map_freq[elem as usize] += 1;     
        }
        // let total_bytes = buffer_in.len();

        for (pos, e) in map_freq.iter().enumerate() {
            if *e != 0 {
                self.vec_elem_count.push(Elem::Leaf(
                    LeafType {
                        pos: pos as u8,
                        count: *e,
                    }
                    ) );
            }
        }

        // self.vec_node_count = map_freq.iter().enumerate()
        //     .filter( |(_pos, e)| **e != 0 )
        //     .map( |(pos, e)| Elem::Leaf( Leaf {pos: pos as u8, count: *e, } ))
        //     .collect();
    }

    /// 3. By using a priority queue and the Huffman coding tree find the best    
    ///    coding for each symbol of the message. Create a table for the code.
    ///    This table inverted will also have to be known in the decoding phase.
    fn generate_huffman_code(& mut self) {
        
        if self.vec_elem_count.len() == 1 {
            if let Elem::Leaf( LeafType {pos, count: _} ) = self.vec_elem_count[0] {
                self.map_encoding.insert( pos, "".to_string() );
                return ();
            }
        }

        while self.vec_elem_count.len() >= 2 {
            self.vec_elem_count.sort_by( 
                | elem_a : &Elem, elem_b : &Elem| 
                {
                    match elem_b {
                        Elem::Node( NodeType { total_count, left: _, right: _} ) => {
                            let total_count_b = total_count;
                            match elem_a {
                                Elem::Node( NodeType { total_count, left: _, right: _} ) => {
                                            let total_count_a = &total_count;
                                            total_count_b.partial_cmp(total_count_a).unwrap()
                                        }
                                Elem::Leaf( LeafType { pos: _, count } ) => {
                                            let count_a = &count;                
                                            total_count_b.partial_cmp(count_a).unwrap()
                                        }    
                                }
                            }
                        Elem::Leaf( LeafType { pos: _, count } ) => {
                            let count_b = &count;
                            match elem_a {
                                Elem::Node( NodeType { total_count, left: _, right: _ } ) => {
                                            let total_count_a = &total_count;
                                            count_b.partial_cmp(total_count_a).unwrap()
                                        }
                                Elem::Leaf( LeafType { pos: _, count } ) => {
                                            let count_a = &count;
                                            count_b.partial_cmp(count_a).unwrap()
                                        }    
                            }
                        } 
                    }
                } );
            // Remove the elements from the end.
            // Search for lowest element.
            let elem_0 = self.vec_elem_count.remove(self.vec_elem_count.len() - 1);
            // Search for the second lowest element.
            let elem_1 = self.vec_elem_count.remove(self.vec_elem_count.len() - 1);

            
            let node = match elem_0 {
                Elem::Node( NodeType { total_count, left: _, right: _} ) => {
                    let total_count_0 = total_count;
                    match elem_1 {
                        Elem::Node( NodeType { total_count, left: _, right: _} ) => {
                                    let total_count_1 = &total_count;
                                    Elem::Node( NodeType{
                                        total_count: total_count_0 + total_count_1,
                                        left:        Box::new(elem_0),
                                        right:       Box::new(elem_1),
                                    })
                                }
                        Elem::Leaf( LeafType { pos: _, count } ) => {
                                    let count_1 = &count;                
                                    Elem::Node( NodeType{
                                        total_count: total_count_0 + count_1,
                                        left:        Box::new(elem_0),
                                        right:       Box::new(elem_1),
                                    })
                                }    
                        }
                    }
                Elem::Leaf( LeafType { pos: _, count } ) => {
                    let count_0 = &count;
                    match elem_1 {
                        Elem::Node( NodeType { total_count, left: _, right: _ } ) => {
                                    let total_count_1 = &total_count;
                                    Elem::Node( NodeType{
                                        total_count: count_0 + total_count_1,
                                        left:        Box::new(elem_0),
                                        right:       Box::new(elem_1),
                                    })
                                }
                        Elem::Leaf( LeafType { pos: _, count } ) => {
                                    let count_1 = &count;
                                    Elem::Node( NodeType{
                                        total_count: count_0 + count_1,
                                        left:        Box::new(elem_0),
                                        right:       Box::new(elem_1),
                                    })
                                }    
                    }
                } 
            };

            self.vec_elem_count.push(node);

        }

        println!("...finished generating huffman code tree!");
        self.print_huffman_tree();
        self.get_huffman_code_from_tree();        
    }

    fn get_huffman_code_from_tree(& mut self) {
        println!("...get_huffman_code_from_tree:");
        let curr_node: & Elem = & self.vec_elem_count[0];
        let start_code = "".to_string();
        let mut map: HashMap< u8, String > = HashMap::new();
        self.transverse_tree_get_huffman_codes(curr_node, start_code, & mut map );
        self.map_encoding = map;
    }

    /// Transverse the tree recursively.
    fn transverse_tree_get_huffman_codes(& self, curr_elem: & Elem, code: String, map_encoding_p: & mut HashMap< u8, String > ) {
        match curr_elem {
            Elem::Node( NodeType { total_count: _, left, right} ) => {
                let new_code_left = code.clone() + "0";
                self.transverse_tree_get_huffman_codes(left,new_code_left, map_encoding_p);
                let new_code_right = code.clone() + "1";
                self.transverse_tree_get_huffman_codes(right,new_code_right, map_encoding_p);
            }
            Elem::Leaf( LeafType { pos, count: _ } ) => {
                let symbol_byte:u8 = *pos;
                let huffman_code = code; 
                map_encoding_p.insert(symbol_byte, huffman_code);
            }
        }
    }

    /// Prints the Huffman coding tree.
    fn print_huffman_tree(& self) {
        println!("...huffman code tree:");

        let curr_node: & Elem = &self.vec_elem_count[0];
        let offset = 0;
        self.print_tree_node(curr_node, offset, false, false, "".to_string() );
    }    

    /// Transverse the tree recursively for printing the nodes and leafs of the tree.
    fn print_tree_node(& self, curr_elem: & Elem, offset: usize, changed_line: bool, other_leaf: bool, code: String) -> bool {
        match curr_elem {
            Elem::Node( NodeType { total_count, left, right} ) => {
                if changed_line {
                    let offset_string = " ".to_string().repeat(offset);
                    print!{"{}", offset_string};
                }
                print!(" |-  {:3} -| ", total_count);
                let new_code_left = code.clone() + "0";
                let ret_flag = self.print_tree_node(left, offset + 12, false, false, new_code_left);
                let new_code_right = code.clone() + "1";
                self.print_tree_node(right,offset + 12, true, ret_flag, new_code_right);
                return false;
            }
            Elem::Leaf( LeafType { pos, count } ) => {
                if other_leaf {
                    let offset_string = " ".to_string().repeat(offset);
                    print!{"{}", offset_string};
                }
                
                if self.print_text_char {
                    let mut c = *pos as char;
                    if c == '\n' {
                        c = '\\';
                    }
                    println!(" <-{} {:3}  {} ->", c, count, code);
                } else {
                    println!(" <-{} {:3}  {} ->", pos, count, code);
                }
                return true;
            }
        }
    }

    /// 4. Write the table to the beginning of byte buffer and the 16 bit heading,
    ///    with the start of the data.
    fn write_mapping_table_to_byte_buffer(& mut self, buffer_out: & mut Vec<u8>) {

        // Fill in the map decoding, from String to u8 byte.
        self.map_decoding = self.map_encoding.iter()
            .map(|(byte_a_start, string_a_end)| {
                let string_b_start = string_a_end.clone();
                let byte_b_end: u8 = byte_a_start.clone();  

                (string_b_start, byte_b_end)
            }).collect();    

        // Fill in the header with zeros.
        buffer_out.push(0);
        buffer_out.push(0);

        let mut vec_tmp: Vec<(String, u8)> = self.map_decoding.iter()
                    .map(|(k, v)| (k.clone(), *v) )
                    .collect();

        vec_tmp.sort_by(|(key_a, _val_a), (key_b, _val_b)| key_a.cmp(key_b));

        for (key, value) in & vec_tmp {
            buffer_out.extend_from_slice(key.as_bytes());
            buffer_out.push('\n' as u8);
            buffer_out.push(*value);
        }

        println!("\n map_decoding: \n{:?}\n\n", vec_tmp);

        // Fill in the header with the position of one plus the end of
        // the header or the position of the start of the compressed data.
        let len = buffer_out.len();
        let len_first: u8 = (len & 0x0000_00FF) as u8;
        let len_second: u8 = ((len & 0x0000_FF00) >> 8) as u8;
        buffer_out[0] = len_second;
        buffer_out[1] = len_first;
    }
    
    /// 5. With the new dictionary, encode the message in bytes to a byte buffer.
    /// 6. Write the first 8 byte with an usize 64 bit's representing the number
    ///    of bytes or total symbols in the original file of the message.    
    fn encode_the_data(&self, buffer_in: & Vec<u8>, buffer_out: & mut Vec<u8>) {

        let start_2_header = buffer_out.len();

        println!("\n...start index of data {} ", start_2_header);

        // Fill in the second header, for the 64 bit, 8 bytes number of symbols,
        // with zeros.
        buffer_out.push(0);
        buffer_out.push(0);
        buffer_out.push(0);
        buffer_out.push(0);

        buffer_out.push(0);
        buffer_out.push(0);
        buffer_out.push(0);
        buffer_out.push(0);

        // Encode from buffer_in into buffer_out_after the the decoding table.
        let mut symbol_counter: usize = 0;  
        let mut index_out_bit: u8     = 0;
        let mut byte_out:u8           = 0b0000_0000;

        for byte in buffer_in {
            symbol_counter += 1;
            // Get the byte.
            // Get the encoding string.
            let string_enc= self.map_encoding.get(byte).unwrap();
        
            // Convert the encoding string into the next bit's in the buffer_out.
            // At the end of each bytes writes to the buffer_out
            for c in string_enc.chars(){
                // print!("{}", c);
                if c == '1' {
                    byte_out |= 0b1000_0000 >> index_out_bit;
                }
                index_out_bit += 1;
                if index_out_bit >= 8 {
                    index_out_bit = 0;
                    buffer_out.push(byte_out);
                    byte_out = 0b0000_0000;
                }
            }
        }
        if index_out_bit > 0 {
            buffer_out.push(byte_out);
        }

        // Debug: 
        // println!();
        // buffer_out[start_2_header + 8 ..].iter().map(|byte| print!("{:b} ", byte)).count();


        // Fill in the second header with the number of symbols in the file data section,
        // or by other words the number of bytes of the original file.
        // We need this second header for the deconding fase, because our compressed 
        // symbols are variable length in size, from zero to n bits.
        
        /*
        let symbol_ct_0: u8 = ((symbol_counter & (0x00FF_usize <<  0)) >>  0) as u8;
        let symbol_ct_1: u8 = ((symbol_counter & (0x00FF_usize <<  8)) >>  8) as u8;
        let symbol_ct_2: u8 = ((symbol_counter & (0x00FF_usize << 16)) >> 16) as u8;
        let symbol_ct_3: u8 = ((symbol_counter & (0x00FF_usize << 24)) >> 24) as u8;
        let symbol_ct_4: u8 = ((symbol_counter & (0x00FF_usize << 32)) >> 32) as u8;
        let symbol_ct_5: u8 = ((symbol_counter & (0x00FF_usize << 40)) >> 40) as u8;
        let symbol_ct_6: u8 = ((symbol_counter & (0x00FF_usize << 48)) >> 48) as u8;
        let symbol_ct_7: u8 = ((symbol_counter & (0x00FF_usize << 56)) >> 56) as u8;
                
        buffer_out[start_2_header + 0] = symbol_ct_7;
        buffer_out[start_2_header + 1] = symbol_ct_6;
        buffer_out[start_2_header + 2] = symbol_ct_5;
        buffer_out[start_2_header + 3] = symbol_ct_4;
        buffer_out[start_2_header + 4] = symbol_ct_3;
        buffer_out[start_2_header + 5] = symbol_ct_2;
        buffer_out[start_2_header + 6] = symbol_ct_1;
        buffer_out[start_2_header + 7] = symbol_ct_0;
        */

        // Compressed version.
        for i in 0..8 {
            let symbol_ct: u8 = ((symbol_counter & (0x00FF_usize <<  (i*8))) >>  (i*8)) as u8;
            buffer_out[start_2_header + 7 - i] = symbol_ct;
        }

        println!("\n...symbol_counter or original file byte size {} ", symbol_counter);

    }

    ///********************
    ///* Decompress methods
    ///********************

    /// 2. Extract the symbols coding table to an internal representation. That is
    ///    the one with the Huffman coding inverted for decoding. 
    fn read_mapping_table_from_byte_buffer(&mut self, buffer_in: & Vec<u8>) -> usize {

        // Read the first header with the position of one plus the end of
        // the header or the position of the start of the compressed data.
        let len_second = buffer_in[0];
        let len_first  = buffer_in[1];
        let header_2_start: usize = (len_second as usize) << 8 | (len_first as usize);  

        println!("\n...header_1_start index in the .johnny compressed input  {} ", header_2_start);


        println!("\n...decoding table:\n");

        let mut string_key_acc = String::new();
        let mut flag_dec_value = false;
        for i in 2..header_2_start {
            let c = buffer_in[i] as char;
            if !flag_dec_value {
                if c == '\n' {
                    flag_dec_value = true;

                } else {
                    string_key_acc.push(c);
                }
            } else {
                flag_dec_value = false;
                let value_byte = buffer_in[i];

                self.map_decoding.insert(string_key_acc.clone(), value_byte);
                if self.print_text_char {
                    // println!("{} -> {}", string_key_acc, value_byte as char);
                } else {
                    println!("{} -> {}", string_key_acc, value_byte);
                }
                string_key_acc.clear();

            }
        }
        
        header_2_start
    }

    // 3. Read the 16 bit header with the index (of the byte) of the start of
    //    the data in the .johnny file. Read the second header with the number
    //    of original symbols, or we could say original bytes. 
    //    Apply the decoding table to the coded message bytes, buffer_in, and decode or
    //    decompress it into a binary buffer_out. 
    fn decode_the_data(&mut self, buffer_in: & Vec<u8>, buffer_out: & mut Vec<u8>, header_2_start: usize) {

        // Read the second header with the number of symbols or bytes of the original
        // file. This is important because the number of bit's for each compressed symbol
        // varies with the symbol and implements exactly a variable minimal Huffman encoding.   

        // Compressed version.
        let mut symbol_counter: usize = 0;
        for i in 0..8 {
            let symbol_ct = buffer_in[header_2_start + 7 - i];
            symbol_counter |= (symbol_ct as usize) << (i*8);
        }

        println!("\n...symbol_counter or original file byte size {} ", symbol_counter);

        // We obtain the data sub_range slice to iterate over it.
        let sub_range_buffer_in = &buffer_in[header_2_start + 8 ..];

        let mut string_key = String::new();
        'outer: for byte in sub_range_buffer_in {
            for index_in_bit in 0_u8..8_u8 {
                let byte_out = (*byte & (0b1000_0000 >> index_in_bit)) >> (7 - index_in_bit);
                if byte_out == 1 {
                    string_key.push('1');
                    // print!("1");
                } else {
                    string_key.push('0');
                    // print!("0");
                }
                if let Some(value_byte) = self.map_decoding.get(& string_key) {
                    string_key.clear();
                    buffer_out.push(*value_byte);
                    // print!("({})", *value_byte as char);
                    
                    // To manage the not full filled last byte.
                    symbol_counter -= 1;
                    if symbol_counter <= 0 {
                        break 'outer;    
                    }
                }
            }
        }
    
        println!();
    }

}
