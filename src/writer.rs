use csv::{Writer, ByteRecord};
use std::io;
// use std::sync::mpsc::Receiver;
use crossbeam_channel::Receiver;


/// Receive data records and print them (usually, to stdout or a file).
/// FIXME rename this to just `writer`.
pub fn writer_thread(rx: Receiver<ByteRecord>) {
    // FIXME switch from hardcoded stdout to a more generic solution
    let mut writer = Writer::from_writer(io::stdout());

    for record in rx {
        writer.write_record(&record).unwrap();
    }

    writer.flush().unwrap();
}
