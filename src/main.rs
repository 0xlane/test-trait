use ferrisetw::native::etw_types::EventRecord;
use ferrisetw::parser::{Parser, TryParse};
use ferrisetw::provider::*;
use ferrisetw::schema::SchemaLocator;
use ferrisetw::trace::*;
use windows::Win32::Foundation::FILETIME;
use std::time::Duration;

fn main() {
    use std::process::Command;
    let _ = Command::new("cmd").args(["/c", "pause"]).status();
    let process_callback =
        |record: EventRecord, schema_locator: &mut SchemaLocator| match schema_locator
            .event_schema(record)
        {
            Ok(schema) => {
                if let 1 = schema.event_id() {
                    println!("{}", schema.process_id());
                    let name = schema.provider_name();
                    println!("Name: {}", name);
                    let mut parser = Parser::create(&schema);
                    let create_time: FILETIME = parser.try_parse("CreateTime").unwrap();
                    let process_id: u32 = parser.try_parse("ProcessID").unwrap();
                    let image_name: String = parser.try_parse("ImageName").unwrap();
                    println!(
                        "PID: {}, ImageName: {}, Low: {}, High: {}",
                        process_id, image_name, create_time.dwLowDateTime, create_time.dwLowDateTime
                    );
                }
            }
            Err(err) => println!("Error {:?}", err),
        };

    let process_provider = Provider::new()
        .by_guid("22fb2cd6-0e7b-422b-a0c7-2fad1fd0e716") // Microsoft-Windows-Kernel-Process
        .add_callback(process_callback)
        .build()
        .unwrap();

    let mut trace = UserTrace::new()
        .named(String::from("MyProvider"))
        .enable(process_provider)
        .start()
        .unwrap();

    std::thread::sleep(Duration::new(20, 0));
    trace.stop();
}