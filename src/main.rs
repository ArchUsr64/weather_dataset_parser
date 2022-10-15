#![allow(unused)]
use quick_csv::*;
use std::fs;

#[derive(Debug)]
struct Time {
	year: u32,
	month: u32,
	date: u32,
	hour: u32,
}
impl Time {
	pub fn time_from_string(data: String) -> Self {
		let year = data[0..4].parse::<u32>().unwrap();
		let month = data[5..7].parse::<u32>().unwrap();
		let date = data[8..10].parse::<u32>().unwrap();
		let hour = data[11..13].parse::<u32>().unwrap();
		Self {
			year,
			month,
			date,
			hour,
		}
	}
}

struct WeatherData {
	formatted_date: Time,
	summary: String,
	precip_kind: String,
	temperature: f32,
	apparent_temperature: f32,
	humidity: f32,
	wind_speed: f32,
	wind_bearing: f32,
	visibility: f32,
	loud_cover: f32,
	pressure: f32,
	daily_summary: String,
}

fn custom_parser(input: &String) -> f32{
	input.parse::<f32>().unwrap()
}

impl WeatherData{
	pub fn from_raw(data: RawCSVData<String>) -> Self{
		WeatherData{
			formatted_date: Time::time_from_string(data.formatted_date),
			summary: data.summary.clone(),
			precip_kind: data.precip_kind.clone(),
			temperature: custom_parser(&data.temperature),
			apparent_temperature: custom_parser(&data.apparent_temperature),
			humidity: custom_parser(&data.humidity),
			wind_speed: custom_parser(&data.wind_speed),
			wind_bearing: custom_parser(&data.wind_bearing),
			visibility: custom_parser(&data.visibility),
			loud_cover: custom_parser(&data.loud_cover),
			pressure: custom_parser(&data.pressure),
			daily_summary: data.daily_summary,
		}
	}
	pub fn print(&self){
		println!("Date: \t[{:?}]", self.formatted_date);
		println!("Day summary: \t{}", self.summary);
		println!("Precipitation Type: \t{}", self.precip_kind);
		println!("Temperature: \t{:0.01}°C", self.temperature);
		println!("Apparent Temperature: \t{:0.01}°C", self.apparent_temperature);
		println!("Humidity: \t{:0.01}%", self.humidity*100f32);
		println!("Wind speed: \t{:0.01}km/hr", self.wind_speed);
		println!("Wind bearing: \t{:0.01}°", self.wind_bearing);
		println!("visibility: \t{:.01}km", self.visibility);
		println!("Loud cover : \t{}", self.loud_cover);
		//wtf does loud cover even mean??
		println!("Pressure: \t{:.02} mb", self.pressure);
		println!("Daily summary: \t{}", self.daily_summary);
	}
}

struct RawCSVData<T> {
	formatted_date: T,
	summary: T,
	precip_kind: T,
	temperature: T,
	apparent_temperature: T,
	humidity: T,
	wind_speed: T,
	wind_bearing: T,
	visibility: T,
	loud_cover: T,
	pressure: T,
	daily_summary: T,
}

impl RawCSVData<String> {
	pub fn new() -> Self {
		RawCSVData {
			formatted_date: String::new(),
			summary: String::new(),
			precip_kind: String::new(),
			temperature: String::new(),
			apparent_temperature: String::new(),
			humidity: String::new(),
			wind_speed: String::new(),
			wind_bearing: String::new(),
			visibility: String::new(),
			loud_cover: String::new(),
			pressure: String::new(),
			daily_summary: String::new(),
		}
	}
	pub fn from_row(&mut self, row: quick_csv::Row) {
		for (i, col) in row.columns().unwrap().enumerate() {
			match i {
				0 => self.formatted_date = String::from(col).clone(),
				1 => self.summary = String::from(col).clone(),
				2 => self.precip_kind = String::from(col).clone(),
				3 => self.temperature = String::from(col).clone(),
				4 => self.apparent_temperature = String::from(col).clone(),
				5 => self.humidity = String::from(col).clone(),
				6 => self.wind_speed = String::from(col).clone(),
				7 => self.wind_bearing = String::from(col).clone(),
				8 => self.visibility = String::from(col).clone(),
				9 => self.loud_cover = String::from(col).clone(),
				10 => self.pressure = String::from(col).clone(),
				11 => self.daily_summary = String::from(col).clone(),
				_ => panic!("Invalid column : [{col}]"),
			}
		}
	}
}

fn main() {
	let file_path = "weatherHistory.csv";
	let data = fs::read_to_string(file_path).expect(file_path);
	let mut total_line_count = 0;
	for _ in data.lines() {
		total_line_count += 1;
	}
	println!("Total line count: [{total_line_count}]");
	let total_line_count = total_line_count;
	let csv = quick_csv::Csv::from_string(&data);
	let mut data = Vec::<WeatherData>::new();

	let mut label = RawCSVData::<String>::new();

	for (i, row) in csv.into_iter().enumerate() {
		match row {
			Ok(row) => {
				if (i == 0) {
					label.from_row(row);
					continue;
				}
				let mut raw_data = RawCSVData::<String>::new();
				raw_data.from_row(row);
				Time::time_from_string(raw_data.formatted_date.clone());
				data.push(WeatherData::from_raw(raw_data));
			}
			Err(_) => {
				row.expect("Failed to parse line number [{line_count}]");
			}
		}
		println!(
			"Parsing... \t[{:.01}%]",
			(100f32 * (i + 1) as f32 / total_line_count as f32)
		);
	}
	println!("{file_path} successfully parsed");
	println!("Nicely printing stuff");
	for (i, data) in data.iter().enumerate(){
		println!("Data at index: {i}");
		data.print();
	}
}
