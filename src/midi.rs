use midly::{Format, Header, MetaMessage, MidiMessage, Smf, Timing, Track, TrackEvent};
use midly::num::{u28, u24, u7, u4};
use std::fs::File;
use std::io::Write;

// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE
// PLACE HOLDER
// AI GENERATED CODE
// DO NOT USE

fn main() {
    // Create a new MIDI file with a single track
    let header = Header::new(
        Format::SingleTrack,
        Timing::Metrical(480.into()) // 480 ticks per quarter note
    );
    let mut smf = Smf::new(header);

    // Create a new track
    let mut track = Track::new();

    // Set the tempo (500,000 microseconds per beat = 120 BPM)
    let tempo = MetaMessage::Tempo(u24::from(500_000));
    track.push(TrackEvent {
        delta: u28::new(0),
        kind: midly::TrackEventKind::Meta(tempo),
    });

    // Add some notes to the track
    let note_on = MidiMessage::NoteOn {
        key: u7::new(60),  // Middle C
        vel: u7::new(64)   // Velocity
    };
    let note_off = MidiMessage::NoteOff {
        key: u7::new(60),
        vel: u7::new(64)
    };

    // Add note on event
    track.push(TrackEvent {
        delta: u28::new(0),
        kind: midly::TrackEventKind::Midi {
            channel: u4::new(0),
            message: note_on
        },
    });

    // Add note off event after 48 ticks (1 quarter note)
    track.push(TrackEvent {
        delta: u28::new(48),
        kind: midly::TrackEventKind::Midi {
            channel: u4::new(0),
            message: note_off
        },
    });

    // Add the track to the MIDI file
    smf.tracks.push(track);

    // Write to a buffer first
    let mut buffer = Vec::new();
    smf.write(&mut buffer).expect("Failed to write to buffer");

    // Write buffer to file
    File::create("output.mid")
        .expect("Failed to create file")
        .write_all(&buffer)
        .expect("Failed to write to file");

    println!("MIDI file created successfully!");
}
