use chrono::{Datelike, NaiveDateTime, NaiveTime, Timelike};

static SHORT_MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
static WEEK_DATS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub struct FormatDateTime;
impl FormatDateTime {
    pub fn normal<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 20 {
            return None;
        }
        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[5] = month[0];
        buf[6] = month[1];
        buf[7] = month[2];
        buf[8] = b'-';
        let day = dt.day();
        buf[9] = ((day / 10) as u8) + 48;
        buf[10] = ((day % 10) as u8) + 48;
        buf[11] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[12] = b' ';
        } else {
            buf[12] = ((hour / 10) as u8) + 48;
        }
        buf[13] = ((hour % 10) as u8) + 48;
        buf[14] = b':';
        let minute = dt.minute();
        buf[15] = ((minute / 10) as u8) + 48;
        buf[16] = ((minute % 10) as u8) + 48;
        buf[17] = b':';
        let second = dt.second();
        buf[18] = ((second / 10) as u8) + 48;
        buf[19] = ((second % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..20]) })
    }
    pub fn full<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 25 {
            return None;
        }

        let wday = WEEK_DATS[dt.weekday() as usize].as_bytes();
        buf[0] = wday[0];
        buf[1] = wday[1];
        buf[2] = wday[2];
        buf[3] = b',';
        buf[4] = b' ';

        let year = dt.year();
        buf[5] = (year / 1000) as u8 + 48;
        buf[6] = ((year % 1000) / 100) as u8 + 48;
        buf[7] = ((year % 100) / 10) as u8 + 48;
        buf[8] = (year % 10) as u8 + 48;
        buf[9] = b'-';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[10] = month[0];
        buf[11] = month[1];
        buf[12] = month[2];
        buf[13] = b'-';
        let day = dt.day();
        buf[14] = ((day / 10) as u8) + 48;
        buf[15] = ((day % 10) as u8) + 48;
        buf[16] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[17] = b' ';
        } else {
            buf[17] = ((hour / 10) as u8) + 48;
        }
        buf[18] = ((hour % 10) as u8) + 48;
        buf[19] = b':';
        let minute = dt.minute();
        buf[20] = ((minute / 10) as u8) + 48;
        buf[21] = ((minute % 10) as u8) + 48;
        buf[22] = b':';
        let second = dt.second();
        buf[23] = ((second / 10) as u8) + 48;
        buf[24] = ((second % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..25]) })
    }
    pub fn short<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 16 {
            return None;
        }
        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b'-';
        let month = dt.month();
        buf[5] = ((month / 10) as u8) + 48;
        buf[6] = ((month % 10) as u8) + 48;
        buf[7] = b'-';
        let day = dt.day();
        buf[8] = ((day / 10) as u8) + 48;
        buf[9] = ((day % 10) as u8) + 48;
        buf[10] = b' ';
        let hour = dt.hour();
        if hour < 10 {
            buf[11] = b' ';
        } else {
            buf[11] = ((hour / 10) as u8) + 48;
        }
        buf[12] = ((hour % 10) as u8) + 48;
        buf[13] = b':';
        let minute = dt.minute();
        buf[14] = ((minute / 10) as u8) + 48;
        buf[15] = ((minute % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..16]) })
    }
}

pub struct FormatTime;
impl FormatTime {
    pub fn short<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 5 {
            return None;
        }
        let hour = dt.hour();
        if hour < 10 {
            buf[0] = b' ';
        } else {
            buf[0] = ((hour / 10) as u8) + 48;
        }
        buf[1] = ((hour % 10) as u8) + 48;
        buf[2] = b':';
        let minute = dt.minute();
        buf[3] = ((minute / 10) as u8) + 48;
        buf[4] = ((minute % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..5]) })
    }
    pub fn am_pm<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 8 {
            return None;
        }
        let hour_24 = dt.hour();
        let hour = if hour_24 > 12 { hour_24 - 12 } else { hour_24 };

        if hour < 10 {
            buf[0] = b' ';
        } else {
            buf[0] = ((hour / 10) as u8) + 48;
        }
        buf[1] = ((hour % 10) as u8) + 48;
        buf[2] = b':';
        let minute = dt.minute();
        buf[3] = ((minute / 10) as u8) + 48;
        buf[4] = ((minute % 10) as u8) + 48;
        buf[5] = b' ';
        if hour_24>=12 {
            buf[6] = b'P';
            buf[7] = b'M';
        } else {
            buf[6] = b'A';
            buf[7] = b'M';
        }

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..8]) })
    }
    pub fn normal<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 8 {
            return None;
        }
        let hour = dt.hour();
        if hour < 10 {
            buf[0] = b' ';
        } else {
            buf[0] = ((hour / 10) as u8) + 48;
        }
        buf[1] = ((hour % 10) as u8) + 48;
        buf[2] = b':';
        let minute = dt.minute();
        buf[3] = ((minute / 10) as u8) + 48;
        buf[4] = ((minute % 10) as u8) + 48;
        buf[5] = b':';
        let seconds = dt.second();
        buf[6] = ((seconds / 10) as u8) + 48;
        buf[7] = ((seconds % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..8]) })
    }
}

pub struct FormatDate;
impl FormatDate {
    pub fn short<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
        todo!()
    }
}
