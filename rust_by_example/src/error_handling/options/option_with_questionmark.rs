pub fn next_birthday(currect_age: Option<u8>) -> Option<String> {
    let next_age: u8 = currect_age?;
    Some(format!("Next year I will be {}", next_age))
}

struct Person {
    job: Option<Job>,
}

#[derive(Clone, Copy)]
struct Job {
    phone_number: Option<PhoneNumber>,
}

#[derive(Copy, Clone)]
struct PhoneNumber {
    area_code: Option<u8>,
    number: u32,
}

impl Person {
    fn work_phone_area_code(&self) -> Option<u8> {
        self.job.unwrap().phone_number.unwrap().area_code // same same
                                                          // self.job?.phone_number?.area_code
    }
}

pub fn run() {
    let p = Person {
        job: Some(Job {
            phone_number: Some(PhoneNumber {
                area_code: Some(61),
                number: 123456789,
            }),
        }),
    };

    assert_eq!(p.work_phone_area_code(), Some(61));
}
