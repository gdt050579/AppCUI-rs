use chrono::{Datelike, Duration, NaiveDate, NaiveDateTime, NaiveTime, Timelike};

static SHORT_MONTHS: [&str; 12] = ["Jan", "Feb", "Mar", "Apr", "May", "Jun", "Jul", "Aug", "Sep", "Oct", "Nov", "Dec"];
static WEEK_DATS: [&str; 7] = ["Mon", "Tue", "Wed", "Thu", "Fri", "Sat", "Sun"];

pub(crate) struct FormatDateTime;
impl FormatDateTime {
    pub(crate) fn normal<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
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
    pub(crate) fn full<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
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
    pub(crate) fn short<'a>(dt: &NaiveDateTime, buf: &'a mut [u8]) -> Option<&'a str> {
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

pub(crate) struct FormatTime;
impl FormatTime {
    pub(crate) fn short<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
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
    pub(crate) fn am_pm<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
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
        if hour_24 >= 12 {
            buf[6] = b'P';
            buf[7] = b'M';
        } else {
            buf[6] = b'A';
            buf[7] = b'M';
        }

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..8]) })
    }
    pub(crate) fn normal<'a>(dt: &NaiveTime, buf: &'a mut [u8]) -> Option<&'a str> {
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

pub(crate) struct FormatDate;
impl FormatDate {
    pub(crate) fn ymd<'a>(dt: &NaiveDate, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 10 {
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

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..10]) })
    }
    pub(crate) fn dmy<'a>(dt: &NaiveDate, buf: &'a mut [u8], separator: u8) -> Option<&'a str> {
        if buf.len() < 10 {
            return None;
        }
        let day = dt.day();
        buf[0] = ((day / 10) as u8) + 48;
        buf[1] = ((day % 10) as u8) + 48;
        buf[2] = separator;
        let month = dt.month();
        buf[3] = ((month / 10) as u8) + 48;
        buf[4] = ((month % 10) as u8) + 48;
        buf[5] = separator;
        let year = dt.year();
        buf[6] = (year / 1000) as u8 + 48;
        buf[7] = ((year % 1000) / 100) as u8 + 48;
        buf[8] = ((year % 100) / 10) as u8 + 48;
        buf[9] = (year % 10) as u8 + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..10]) })
    }

    pub(crate) fn short<'a>(dt: &NaiveDate, buf: &'a mut [u8], separator: u8) -> Option<&'a str> {
        if buf.len() < 8 {
            return None;
        }
        let day = dt.day();
        buf[0] = ((day / 10) as u8) + 48;
        buf[1] = ((day % 10) as u8) + 48;
        buf[2] = separator;
        let month = dt.month();
        buf[3] = ((month / 10) as u8) + 48;
        buf[4] = ((month % 10) as u8) + 48;
        buf[5] = separator;
        let year = dt.year() % 100;
        buf[6] = (year / 10) as u8 + 48;
        buf[7] = (year % 10) as u8 + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..8]) })
    }

    pub(crate) fn normal<'a>(dt: &NaiveDate, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 13 {
            return None;
        }

        let year = dt.year();
        buf[0] = (year / 1000) as u8 + 48;
        buf[1] = ((year % 1000) / 100) as u8 + 48;
        buf[2] = ((year % 100) / 10) as u8 + 48;
        buf[3] = (year % 10) as u8 + 48;
        buf[4] = b',';
        buf[5] = b' ';
        let month = SHORT_MONTHS[dt.month().saturating_sub(1) as usize].as_bytes();
        buf[6] = month[0];
        buf[7] = month[1];
        buf[8] = month[2];
        buf[9] = b',';
        buf[10] = b' ';
        let day = dt.day();
        buf[11] = ((day / 10) as u8) + 48;
        buf[12] = ((day % 10) as u8) + 48;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..13]) })
    }

    pub(crate) fn full<'a>(dt: &NaiveDate, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 16 {
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

        Some(unsafe { core::str::from_utf8_unchecked(&buf[..16]) })
    }
}

pub(crate) struct FormatDuration;
impl FormatDuration {
    pub(crate) fn auto_hms<'a>(d: &Duration, buf: &'a mut [u8]) -> Option<&'a str> {
        let mut seconds = d.num_seconds();
        if seconds < 60 {
            if buf.len() < 3 {
                return None;
            }
            buf[0] = b':';
            buf[1] = ((seconds / 10) as u8) + 48;
            buf[2] = ((seconds % 10) as u8) + 48;
            return Some(unsafe { core::str::from_utf8_unchecked(&buf[..3]) });
        }
        let mut minutes = seconds / 60;
        seconds %= 60;
        if minutes < 60 {
            if minutes < 10 {
                if buf.len() < 4 {
                    return None;
                }
                buf[0] = (minutes as u8) + 48;
                buf[1] = b':';
                buf[2] = ((seconds / 10) as u8) + 48;
                buf[3] = ((seconds % 10) as u8) + 48;
                return Some(unsafe { core::str::from_utf8_unchecked(&buf[..4]) });
            } else {
                if buf.len() < 5 {
                    return None;
                }
                buf[0] = ((minutes / 10) as u8) + 48;
                buf[1] = ((minutes % 10) as u8) + 48;
                buf[2] = b':';
                buf[3] = ((seconds / 10) as u8) + 48;
                buf[4] = ((seconds % 10) as u8) + 48;
                return Some(unsafe { core::str::from_utf8_unchecked(&buf[..5]) });
            }
        }
        let mut hours = minutes / 60;
        minutes %= 60;
        if buf.len() < 7 {
            return None;
        }
        let mut index = buf.len() - 1;
        buf[index] = ((seconds % 10) as u8) + 48;
        index -= 1;
        buf[index] = ((seconds / 10) as u8) + 48;
        index -= 1;
        buf[index] = b':';
        index -= 1;
        buf[index] = ((minutes % 10) as u8) + 48;
        index -= 1;
        buf[index] = ((minutes / 10) as u8) + 48;
        index -= 1;
        buf[index] = b':';
        index -= 1;
        loop {
            buf[index] = ((hours % 10) as u8) + 48;
            hours /= 10;
            if hours == 0 {
                break;
            }
            if index == 0 {
                return None;
            }
            index -= 1;
        }
        Some(unsafe { core::str::from_utf8_unchecked(&buf[index..]) })
    }
    #[inline(always)]
    fn write_number(value: i64, suffix: u8, pos: usize, output: &mut [u8]) -> Option<usize> {
        if output.len() <= pos {
            return None;
        }
        output[pos] = suffix;
        if pos == 0 {
            return None;
        }
        let mut index = pos - 1;
        let mut value = value;
        loop {
            output[index] = ((value % 10) as u8) + 48;
            value /= 10;
            if value == 0 {
                break;
            }
            if index == 0 {
                return None;
            }
            index -= 1;
        }
        Some(index)
    }
    #[inline(always)]
    fn write_space_ahead(index: usize, output: &mut [u8]) -> Option<usize> {
        if index < 2 {
            return None;
        }
        output[index - 1] = b' ';
        Some(index - 2)
    }
    pub(crate) fn details<'a>(d: &Duration, buf: &'a mut [u8]) -> Option<&'a str> {
        if buf.len() < 2 {
            return None;
        }
        let mut index = buf.len() - 1;
        let mut seconds = d.num_seconds();

        // just seconds
        if seconds < 60 {
            index = FormatDuration::write_number(seconds, b's', index, buf)?;
            return Some(unsafe { core::str::from_utf8_unchecked(&buf[index..]) });
        }
        let mut minutes = seconds / 60;
        seconds %= 60;
        index = FormatDuration::write_number(seconds, b's', index, buf)?;
        index = FormatDuration::write_space_ahead(index, buf)?;

        // just minutes and seconds
        if minutes < 60 {
            index = FormatDuration::write_number(minutes, b'm', index, buf)?;
            return Some(unsafe { core::str::from_utf8_unchecked(&buf[index..]) });
        }
        let mut hours = minutes / 60;
        minutes %= 60;
        index = FormatDuration::write_number(minutes, b'm', index, buf)?;
        index = FormatDuration::write_space_ahead(index, buf)?;

        // just hours, minutes and seconds
        if hours < 24 {
            index = FormatDuration::write_number(hours, b'h', index, buf)?;
            return Some(unsafe { core::str::from_utf8_unchecked(&buf[index..]) });
        }
        let days = hours / 24;
        hours %= 24;
        index = FormatDuration::write_number(hours, b'h', index, buf)?;
        index = FormatDuration::write_space_ahead(index, buf)?;
        index = FormatDuration::write_number(days, b'd', index, buf)?;

        Some(unsafe { core::str::from_utf8_unchecked(&buf[index..]) })
    }
}
