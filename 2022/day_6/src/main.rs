use std::fs;

struct DataStream {
    stream: String,
    start_marker_length: u32,
    start_marker_index: u32,
}

impl DataStream {
    const LCASE_ASCII_OFFSET: u32 = 97;
    const UCASE_ASCII_OFFSET: u32 = 65;

    fn new(stream: String) -> Self {
        return Self {
            stream,
            start_marker_length: 0,
            start_marker_index: 0,
        };
    }

    fn search_marker(&mut self, start_marker_length: u32) {
        assert!(start_marker_length > 0);

        let stream_windows = self.stream.as_bytes().windows(start_marker_length as usize);
        let mut char_lookup: u32 = 0;

        self.start_marker_length = start_marker_length;

        for (index, window) in stream_windows.enumerate() {
            let mut window_pos_counter: u32 = 0;

            for byte in window.iter() {
                let ascii_offset = if (*byte as char).is_lowercase() {
                    DataStream::LCASE_ASCII_OFFSET
                } else {
                    DataStream::UCASE_ASCII_OFFSET
                };
                let mask = 1 << (*byte as u32 - ascii_offset);

                if (char_lookup & mask) >> (*byte as u32 - ascii_offset) == 1 {
                    char_lookup = 0;
                    break;
                }

                char_lookup |= mask;
                window_pos_counter += 1;
            }

            if char_lookup != 0 {
                self.start_marker_index = index as u32 + window_pos_counter;
                break;
            }
        }
    }
}

fn main() {
    let mut data_stream_packets = vec![];
    let mut data_stream_messages = vec![];
    let input = fs::read_to_string("input_p1").expect("Unable to read file");
    let input_lines = input.lines();

    for line in input_lines {
        let mut data_stream_packet = DataStream::new(line.to_string());
        let mut data_stream_message = DataStream::new(line.to_string());
        data_stream_packet.search_marker(4);
        data_stream_message.search_marker(14);
        data_stream_packets.push(data_stream_packet);
        data_stream_messages.push(data_stream_message);
    }

    // Part 1
    println!(
        "Part 1: {:?}",
        data_stream_packets
            .iter()
            .map(|x| (x.stream.to_string(), x.start_marker_index))
            .collect::<Vec<(String, u32)>>()
    );

    // Part 2
    println!(
        "Part 2: {:?}",
        data_stream_messages
            .iter()
            .map(|x| (x.stream.to_string(), x.start_marker_index))
            .collect::<Vec<(String, u32)>>()
    );
}
