use crate::*;

impl Reservation {
    pub fn new_pending<'a>(
        // id: impl Into<String>,
        user_id: impl Into<String>,
        resource_id: impl Into<String>,
        // status: impl Into<i32>,
        start: impl Into<&'a str>,
        end: impl Into<&'a str>,
        note: impl Into<String>,
    ) -> Self {
        Self {
            id: "".into(),
            resource_id: resource_id.into(),
            status: ReservationStatus::Pending as i32,
            user_id: user_id.into(),
            start: Some(convert_str_to_timestamp(start.into())),
            end: Some(convert_str_to_timestamp(end.into())),
            note: note.into(),
        }
    }

    pub fn validate(&self) -> Result<(), Error> {
        if self.user_id.is_empty() {
            return Err(Error::InvalidUserId(&self.user_id));
        }

        if self.resource_id.is_empty() {
            return Err(Error::InvalidResourceId(&self.resource_id));
        }

        if self.start.is_none() || self.end.is_none() {
            return Err(Error::InvalidTime(
                convert_timestamp_to_str(self.start.clone().unwrap()),
                convert_timestamp_to_str(self.end.clone().unwrap()),
            ));
        } else {
            let start = self.start.clone().unwrap();
            let end = self.end.clone().unwrap();
            if start.seconds >= end.seconds {
                return Err(Error::InvalidTime(
                    convert_timestamp_to_str(start),
                    convert_timestamp_to_str(end),
                ));
            }
        }

        Ok(())
    }

    pub fn get_timespan(&self) -> String {
        let start = convert_timestamp_to_naiveDt(self.start.clone().unwrap());
        let end = convert_timestamp_to_naiveDt(self.end.clone().unwrap());

        format!("[{}, {})", start, end)
    }
}
