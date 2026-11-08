use appcui::prelude::*;
use chrono::{Local, NaiveTime};

#[Window(events = ButtonEvents)]
struct BookingForm {
    name: Handle<TextField>,
    date: Handle<DatePicker>,
    time: Handle<TimePicker>,
    service: Handle<ComboBox>,
    people: Handle<NumericSelector<u32>>,
    book: Handle<Button>,
}

impl BookingForm {
    fn new() -> Self {
        let mut win = BookingForm {
            base: window!("'Book an appointment',a:c,w:50,h:15"),
            name: Handle::None,
            date: Handle::None,
            time: Handle::None,
            service: Handle::None,
            people: Handle::None,
            book: Handle::None,
        };

        win.add(label!("'Name:',x:1,y:1,w:12,h:1"));
        win.name = win.add(textfield!("x:14,y:1,w:33,h:1"));

        win.add(label!("'Date:',x:1,y:3,w:12,h:1"));
        let today = Local::now().date_naive();
        win.date = win.add(DatePicker::with_date(today, layout!("x:14,y:3,w:19")));

        win.add(label!("'Time:',x:1,y:5,w:12,h:1"));
        let ten_am = NaiveTime::from_hms_opt(10, 0, 0).unwrap();
        win.time = win.add(TimePicker::with_time(ten_am, layout!("x:14,y:5,w:10"), timepicker::Flags::None));

        win.add(label!("'Service:',x:1,y:7,w:12,h:1"));
        let mut service = ComboBox::new(layout!("x:14,y:7,w:33"), combobox::Flags::ShowDescription);
        service.add_item(combobox::Item::new("Haircut", "(30 min)"));
        service.add_item(combobox::Item::new("Massage", "(60 min)"));
        service.add_item(combobox::Item::new("Consultation", "(45 min)"));
        service.add_item(combobox::Item::new("Dental check-up", "(20 min)"));
        service.set_index(0);
        win.service = win.add(service);

        win.add(label!("'People:',x:1,y:9,w:12,h:1"));
        win.people = win.add(numericselector!("u32,1,1,10,step:1,x:14,y:9,w:19,h:1"));

        win.book = win.add(button!("'&Book',x:18,y:11,w:12"));
        win
    }

    fn book(&mut self) {
        let name = self.control(self.name).map(|t| t.text().trim().to_string()).unwrap_or_default();
        if name.is_empty() {
            dialogs::error("Booking refused", "Please enter a name before booking.");
            let h = self.name;
            self.request_focus_for_control(h);
            return;
        }
        let date = self.control(self.date).map(|d| d.date()).unwrap_or_default();
        let time = self.control(self.time).map(|t| t.time()).unwrap_or_default();
        let service = self
            .control(self.service)
            .and_then(|c| c.selected_item())
            .map(|item| item.value().to_string())
            .unwrap_or_else(|| String::from("(none)"));
        let people = self.control(self.people).map(|n| n.value()).unwrap_or(1);

        let summary = format!(
            "Name: {}\nDate: {}\nTime: {}\nService: {}\nPeople: {}",
            name,
            date.format("%Y-%m-%d"),
            time.format("%H:%M"),
            service,
            people
        );
        dialogs::message("Confirmed", &summary);
    }
}

impl ButtonEvents for BookingForm {
    fn on_pressed(&mut self, handle: Handle<Button>) -> EventProcessStatus {
        if handle == self.book {
            self.book();
            return EventProcessStatus::Processed;
        }
        EventProcessStatus::Ignored
    }
}

fn main() -> Result<(), appcui::system::Error> {
    App::new().window(BookingForm::new).run()
}
