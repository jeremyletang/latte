// Copyright 2016 Jeremy Letang.
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use db::models::Message;

const DAY_SECONDS: i32 = 86400;

// return 0 if the we did not change the day
// return -1 if we need to change all days in the past
// return 1 if we need to increase the days
fn utc_seconds(mut seconds: i32, offset: i32) -> (i32, i8) {
    let mut day_change = 0;
    if offset == 0 {
        // nothing to do
    }
    // we are before utc, so we need to had the inverse of the offset
    // to get the utc time
    else if offset < 0 {
        // add the inverse offset to the actual seconds to make the utc time
        seconds = seconds + (-offset);
        // new seconds are superior to a day, so we change of day
        // e.g: we are at utc-3, and try to set a reminder at 23:00
        // in utc time it will be 2:00 of the next day
        if seconds > DAY_SECONDS {
            seconds = seconds - DAY_SECONDS;
            day_change = 1
        } else {
            // we are still in the same day based on utc time
            // nothing to change
        }
    }
    // we are after utc, so we need to substract the offset to get utc time
    else if offset > 0 {
        seconds = seconds - offset;
        // new seconds are less than 0, so we need to jump to the previous day
        // e.g we are at utc+3, an try to set a reminder at 1:00
        // in utc time it will be 22:00 of the previous day
        if seconds < 0 {
            seconds = DAY_SECONDS + (seconds);
            day_change = -1
        }
    }

    return (seconds, day_change);
}

// return 0 if the we did not change the day
// return -1 if we need to change all days in the past
// return 1 if we need to increase the days
fn timezone_seconds(mut seconds: i32, offset: i32) -> (i32, i8) {
    let mut day_change = 0;
    if offset == 0 {
        // nothing to do
    }
    // we are before utc, so we need to substract the seconds of the offset to get
    // back the our for ou timezone
    else if offset < 0 {
        // add the inverse offset to the actual seconds to make the utc time
        seconds = seconds + offset;
        // new seconds are less than 0
        if seconds < 0 {
            seconds = DAY_SECONDS + seconds;
            day_change = -1
        } else {
            // we are still in the same day based on utc time
            // nothing to change
        }
    }
    // we are after utc, so we need to add the offset to our current value
    else if offset > 0 {
        seconds = seconds + offset;
        // new seconds are more than a day
        if seconds > DAY_SECONDS {
            seconds =  seconds - DAY_SECONDS;
            day_change = 1
        }
    }

    return (seconds, day_change);
}

fn increase_day_in_message(m: &mut Message) {
    let m_copy = m.clone();
    fn apply(a: &mut Option<i32>, b: &mut Option<i32>) {
        *a = Some(0);
        *b = Some(1);
    }
    if m_copy.monday == Some(1) { apply(&mut m.monday, &mut m.tuesday); }
    if m_copy.tuesday == Some(1) { apply(&mut m.tuesday, &mut m.wednesday); }
    if m_copy.wednesday == Some(1) { apply(&mut m.wednesday, &mut m.thursday); }
    if m_copy.thursday == Some(1) { apply(&mut m.thursday, &mut m.friday); }
    if m_copy.friday == Some(1) { apply(&mut m.friday, &mut m.saturday); }
    if m_copy.saturday == Some(1) { apply(&mut m.saturday, &mut m.sunday); }
    if m_copy.sunday == Some(1) { apply(&mut m.sunday, &mut m.monday); }
}

fn decrease_day_in_message(m: &mut Message) {
    let m_copy = m.clone();
    fn apply(a: &mut Option<i32>, b: &mut Option<i32>) {
        *a = Some(0);
        *b = Some(1);
    }
    if m_copy.sunday == Some(1) { apply(&mut m.sunday, &mut m.saturday); }
    if m_copy.saturday == Some(1) { apply(&mut m.saturday, &mut m.friday); }
    if m_copy.friday == Some(1) { apply(&mut m.friday, &mut m.thursday); }
    if m_copy.thursday == Some(1) { apply(&mut m.thursday, &mut m.wednesday); }
    if m_copy.wednesday == Some(1) { apply(&mut m.wednesday, &mut m.tuesday); }
    if m_copy.tuesday == Some(1) { apply(&mut m.tuesday, &mut m.monday); }
    if m_copy.monday == Some(1) { apply(&mut m.monday, &mut m.sunday); }
}

// convert a message with seconds and weekdays in the client timezone
// to utc values
pub fn local_message_to_utc_message(mut m: Message) -> Message {
    let (new_seconds, day_change) = utc_seconds(m.seconds, m.utc_offset);
    m.seconds = new_seconds;
    // check if we need to update the days to apply the reminder
    // dependening of the utc time
    if day_change == -1 {
        decrease_day_in_message(&mut m);
    } else if day_change == 1 {
        increase_day_in_message(&mut m);
    }
    return m;
}

// convert a message with seconds and weekdays in utc based
// to client timezone back
pub fn utc_message_to_local_message(mut m: Message) -> Message {
    let (new_seconds, day_change) = timezone_seconds(m.seconds, m.utc_offset);
    m.seconds = new_seconds;
    // check if we need to update the days to apply the reminder
    // dependening of the utc time
    if day_change == -1 {
        decrease_day_in_message(&mut m);
    } else if day_change == 1 {
        increase_day_in_message(&mut m);
    }
    return m;
}
