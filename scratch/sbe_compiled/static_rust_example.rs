use std::any::Any;
use std::error::Error;

use rscodec::{
    boolean_type::BooleanType,
    car_codec::{decoder::*, encoder::*},
    message_header_codec::MessageHeaderDecoder,
    model::Model,
    optional_extras::OptionalExtras,
    *,
};

use rscodec::car_codec;
use rscodec::Encoder;
use rscodec::WriteBuf;
use rscodec::{SBE_SCHEMA_ID, SBE_SCHEMA_VERSION};

fn main() -> Result<(), Box<dyn Error>>{
    let mut buffer: Vec<u8> = vec![0u8; 1024];

    let encoded_length = encode_car(&mut buffer).ok().ok_or("err")?;
    let shortened_state = &buffer[..encoded_length];
    println!("{:?}", shortened_state);

    let decode_length = decode_car(&buffer)?;
    println!("{:?}", decode_length);

    println!("{}=={}? {}", encoded_length, decode_length, encoded_length == decode_length);
    Ok(())
}

const MANUFACTURER: &str = "MANUFACTURER";
const MODEL: &str = "MODEL";

struct State {
    buffer: Vec<u8>,
}

fn encode_car(buffer: &mut Vec<u8>) -> SbeResult<usize> {
    let buffer = buffer.as_mut_slice();
    let mut car = CarEncoder::default();
    println!("car: {:?}", car);
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
    car.vehicle_code(&[b'1', b'2', b'3', b'4', b'5', b'6']); // abcdef
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


fn decode_car(buffer: &Vec<u8>) -> SbeResult<usize> {
    let mut car = CarDecoder::default();

    let buf = ReadBuf::new(buffer.as_slice());
    let header = MessageHeaderDecoder::default().wrap(buf, 0);
    car = car.header(header, 0);

    // Car...
    car.serial_number();
    car.model_year();
    car.available();
    car.code();

    let len = car.some_numbers().len();
    for i in 0..len {
        let _ = car.some_numbers()[i];
    }

    let len = car.vehicle_code().len();
    for i in 0..len {
        let _ = car.vehicle_code()[i];
    }

    let extras = car.extras();
    extras.get_cruise_control();
    extras.get_sports_pack();
    extras.get_sun_roof();
    println!("{:?}", extras);

    let mut engine = car.engine_decoder();
    engine.capacity();
    engine.num_cylinders();
    engine.max_rpm();
    engine.manufacturer_code();
    engine.fuel();

    car = engine.parent()?;
    let mut fuel_figures = car.fuel_figures_decoder();

    while let Ok(Some(_)) = fuel_figures.advance() {
        fuel_figures.speed();
        fuel_figures.mpg();
    }

    car = fuel_figures.parent()?;
    let mut performance_figures = car.performance_figures_decoder();

    while let Ok(Some(_)) = performance_figures.advance() {
        performance_figures.octane_rating();
        let mut acceleration = performance_figures.acceleration_decoder();
        while let Ok(Some(_)) = acceleration.advance() {
            acceleration.mph();
            acceleration.seconds();
        }
        performance_figures = acceleration.parent()?;
    }

    car = performance_figures.parent()?;
    let coord = car.manufacturer_decoder();
    car.manufacturer_slice(coord);
    let coord = car.model_decoder();
    car.model_slice(coord);

    Ok(car.encoded_length())
}
