use std::any::Any;
use std::error::Error;

use static_rust_codec::{
    boolean_type::BooleanType,
    car_codec::{decoder::*, encoder::*},
    message_header_codec::MessageHeaderDecoder,
    model::Model,
    optional_extras::OptionalExtras,
    *,
};

use static_rust_codec::car_codec;
use static_rust_codec::Encoder;
use static_rust_codec::WriteBuf;
use static_rust_codec::{SBE_SCHEMA_ID, SBE_SCHEMA_VERSION};

fn main() -> Result<(), Box<dyn Error>>{
    let mut state = State {
        buffer: vec![0u8; 1024],
    };
    let sbe_result = encode(&mut state).ok().ok_or("err")?;

    let shortened_state = &state.buffer[..sbe_result];
    println!("{:?}", shortened_state);
    Ok(())
}

// use criterion::{black_box, criterion_group, criterion_main, Criterion};

const MANUFACTURER: &str = "MANUFACTURER";
const MODEL: &str = "MODEL";

struct State {
    buffer: Vec<u8>,
}

fn encode(state: &mut State) -> SbeResult<usize> {
    let buffer = state.buffer.as_mut_slice();
    let mut car = CarEncoder::default();
    let mut fuel_figures = FuelFiguresEncoder::default();
    let mut performance_figures = PerformanceFiguresEncoder::default();
    let mut acceleration = AccelerationEncoder::default();
    let mut extras = OptionalExtras::default();

    car = car.wrap(WriteBuf::new(buffer), message_header_codec::ENCODED_LENGTH);
    car = car.header(0).parent()?;

    car.code(Model::A);
    car.model_year(2005);
    car.serial_number(12345);
    car.available(BooleanType::T);
    car.vehicle_code(&[97, 98, 99, 100, 101, 102]); // abcdef
    car.some_numbers(&[0, 1, 2, 3]);

    extras.set_sports_pack(true);
    extras.set_sun_roof(true);
    car.extras(extras);

    let mut engine = car.engine_encoder();
    engine.capacity(4200);
    engine.num_cylinders(8);
    engine.manufacturer_code(&[97, 98, 99]); // abc

    car = engine.parent()?;
    fuel_figures = car.fuel_figures_encoder(3, fuel_figures);
    fuel_figures.advance()?;
    fuel_figures.speed(30);
    fuel_figures.mpg(35.9);

    fuel_figures.advance()?;
    fuel_figures.speed(55);
    fuel_figures.mpg(49.0);

    fuel_figures.advance()?;
    fuel_figures.speed(75);
    fuel_figures.mpg(40.0);

    car = fuel_figures.parent()?;
    performance_figures = car.performance_figures_encoder(2, performance_figures);
    performance_figures.advance()?;
    performance_figures.octane_rating(95);

    acceleration = performance_figures.acceleration_encoder(3, acceleration);
    acceleration.advance()?;
    acceleration.mph(30);
    acceleration.seconds(4.0);

    acceleration.advance()?;
    acceleration.mph(60);
    acceleration.seconds(7.5);

    acceleration.advance()?;
    acceleration.mph(100);
    acceleration.seconds(12.2);

    performance_figures = acceleration.parent()?;
    performance_figures.advance()?;
    performance_figures.octane_rating(99);

    acceleration = performance_figures.acceleration_encoder(3, acceleration);
    acceleration.advance()?;
    acceleration.mph(30);
    acceleration.seconds(3.8);

    acceleration.advance()?;
    acceleration.mph(60);
    acceleration.seconds(7.1);

    acceleration.advance()?;
    acceleration.mph(100);
    acceleration.seconds(11.8);

    performance_figures = acceleration.parent()?;
    car = performance_figures.parent()?;

    car.manufacturer(MANUFACTURER);
    car.model(MODEL);

    Ok(car.encoded_length())
}