#[allow(unused_variable)] 
#[allow(dead_code)]
enum Pitch {
	Frequency(f64,FrequencyUnits),
	Period(f64,PeriodUnits),
}

impl Show for Pitch {
	

#[allow(dead_code)]
enum FrequencyUnits {
	Hertz,
	BeatsPerMinute,
}

#[allow(dead_code)]
enum PeriodUnits {
	MilliSeconds,
	Seconds,
}

#[allow(dead_code)]
enum SemanticDirection {
	Up,
	Down,
}

#[allow(dead_code)]
enum IntervalUnits {
	FrequencyRatio,
	LogFrequncyDifference,
}

#[allow(dead_code)]
enum SemanticInterval {
	PerfectUnison,
	MinorSecond,
	MajorSecond,
	MinorThird,
	MajorThird,
	PerfectFourth,
	Tritone,
	PerfectFifth,
	MinorSixth,
	MajorSixth,
	MinorSeventh,
	MajorSeventh,
	PerfectOctave,
}

#[allow(dead_code)]
fn to_semitones(interval: SemanticInterval) -> int { interval as int}
		

#[allow(unused_variable)] 
#[allow(dead_code)]
fn main() {
	let pitch1 = Frequency(440.0,Hertz);
	let pitch2 = Period(10.0, MilliSeconds);
	
	//for interval in SemanticIntervals.iter();

}
