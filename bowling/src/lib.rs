use std::vec;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    NotEnoughPinsLeft,
    GameComplete,
}

#[derive(PartialEq, Eq, Clone, Copy)]
enum FrameType {
    Strike,
    Spare,
    Open,
}

struct Frame {
    frame_type: FrameType,
    roll_count: u16,
    pins: u16,
    throws: Vec<u16>,
    score: u16,
    is_done: bool,
}

pub struct BowlingGame {
    frames: Vec<Frame>,
    frame_index: usize,
}

impl BowlingGame {
    pub fn new() -> Self {
        BowlingGame {
            frames: (0..10)
                .map(|_f| Frame {
                    frame_type: FrameType::Open,
                    roll_count: 0,
                    pins: 10,
                    throws: vec![],
                    score: 0,
                    is_done: false,
                })
                .collect(),
            frame_index: 0,
        }
    }

    pub fn roll(&mut self, pins: u16) -> Result<(), Error> {
        if self.is_game_done() {
            return Err(Error::GameComplete);
        }

        let result = match self.frame_index {
            9 => self.on_final_frame(pins),
            _ => self.on_intermediate_frame(pins),
        };

        if self.is_game_done() {
            let frames = &mut self.frames;

            for i in 0..self.frame_index {
                let slice = &mut frames[i..];
                let mut slice_iter = slice.iter_mut();
                let frame = slice_iter.next().unwrap();
                let next_frame = slice_iter.next().unwrap();

                match frame.frame_type {
                    FrameType::Strike => {
                        if next_frame.frame_type == FrameType::Strike {
                            if i != self.frame_index - 1 {
                                let frame_after = slice_iter.next().unwrap();
                                let second_throw = next_frame.throws.iter().next_back().unwrap();
                                let third_throw = frame_after.throws[0];
                                frame.score = 10 + second_throw + third_throw;
                            } else {
                                frame.score = 10 + next_frame.throws[0] + next_frame.throws[1];
                            }
                        } else {
                            frame.score = 10 + next_frame.throws.iter().sum::<u16>();
                        }
                    }
                    FrameType::Spare => {
                        frame.score = 10 + next_frame.throws[0];
                    }
                    FrameType::Open => frame.score = frame.throws.iter().sum(),
                }
            }
        }

        result
    }

    pub fn score(&self) -> Option<u16> {
        if self.is_game_done() {
            let sum = self.frames.iter().fold(0, |mut sum, frame| {
                sum += frame.score;
                sum
            });

            return Some(sum);
        }

        None
    }

    fn is_game_done(&self) -> bool {
        self.frames.iter().all(|frame| frame.is_done)
    }

    fn on_intermediate_frame(&mut self, pins: u16) -> Result<(), Error> {
        let current_frame = &mut self.frames[self.frame_index];
        let result = Self::process_frame(current_frame, self.frame_index, pins);

        match result {
            Ok(frame_type) => {
                current_frame.frame_type = frame_type;
                if current_frame.pins == 0 || current_frame.roll_count == 2 {
                    current_frame.is_done = true;
                    self.frame_index += 1;
                }

                Ok(())
            }
            Err(error) => return Err(error),
        }
    }

    fn on_final_frame(&mut self, pins: u16) -> Result<(), Error> {
        let current_frame = &mut self.frames[self.frame_index];

        let result = Self::process_frame(current_frame, self.frame_index, pins);

        match result {
            Ok(frame_type) => {
                current_frame.frame_type = frame_type;
                if current_frame.frame_type == FrameType::Open && current_frame.roll_count == 2 {
                    current_frame.score = current_frame.throws.iter().sum();
                    current_frame.is_done = true;
                } else if current_frame.frame_type != FrameType::Open {
                    if current_frame.roll_count <= 3 {
                        // Fill ball
                        if current_frame.frame_type == FrameType::Spare
                            && current_frame.roll_count == 2
                        {
                            current_frame.score = 10 - pins;
                        }
                        current_frame.score += pins;
                        if current_frame.pins == 0 {
                            current_frame.pins = 10;
                        }

                        if current_frame.roll_count == 3 {
                            current_frame.is_done = true;
                        }
                    }
                }
            }
            Err(error) => return Err(error),
        }

        Ok(())
    }

    fn get_frame_type(roll_count: u16, pins_remaining: u16) -> FrameType {
        match (roll_count, pins_remaining) {
            (1, 0) => FrameType::Strike,
            (2, 0) => FrameType::Spare,
            _ => FrameType::Open,
        }
    }

    fn process_frame(frame: &mut Frame, frame_index: usize, pins: u16) -> Result<FrameType, Error> {
        if frame.pins < pins {
            return Err(Error::NotEnoughPinsLeft);
        }

        frame.roll_count += 1;
        frame.pins -= pins;
        frame.throws.push(pins);

        if frame_index == 9 && frame.frame_type != FrameType::Open && frame.roll_count > 1 {
            Ok(frame.frame_type)
        } else {
            Ok(Self::get_frame_type(frame.roll_count, frame.pins))
        }
    }
}
