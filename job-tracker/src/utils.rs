use crate::models::{Department, GreenHouseResponse, Job, Location, Office};

#[allow(dead_code)]
pub fn generate_greenhouse_mock() -> GreenHouseResponse {
    GreenHouseResponse {
        jobs: vec![Job {
            education: Some(String::from("education_optional")),
            internal_job_id: 5530957002,
            location: Location {
                name: String::from("San Francisco"),
            },
            id: 6698759002,
            updated_at: String::from("2023-06-07T20:09:17-04:00"),
            requisition_id: String::from("2021-P3-4815-2"),
            title: String::from("Business Systems Analyst (Workday)"),
            departments: vec![Department {
                name: String::from("People Operations"),
            }],
            offices: vec![Office {
                name: String::from("San Francisco, CA or Remote (U.S.)"),
                location: Some(String::from("San Francisco, CA or Remote (U.S.)")),
            }],
        }],
    }
}
