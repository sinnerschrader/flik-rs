use pyo3::{GILGuard, ObjectProtocol, PyDict, PyErr, PyObjectRef, PyResult, Python};
use serde;
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    personID: i32,
    sessionID: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct ProjectList {
    projects: Vec<Project>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Project {
    projectID: i64,
    name: String,
    startTime: String,
    endTime: String,
    billable: bool,
    taskCompulsory: bool,
    commentCompulsory: bool,
    indirectCostCenterAllowed: bool,
}

pub struct BlueantService<'a> {
    python: Python<'a>,
    base_client: &'a PyObjectRef,
    worktime_accounting_client: &'a PyObjectRef,
}

#[derive(Debug)]
pub enum Error {
    PythonErr,
    JsonError,
}

impl<'a> BlueantService<'a> {
    pub fn get_python_gil() -> GILGuard {
        Python::acquire_gil()
    }

    pub fn new(gil: &'a GILGuard) -> Self {
        let python = gil.python();
        let zeep_module = python.import("zeep").unwrap();
        let locals = PyDict::new(python);
        locals.set_item("zeep", zeep_module).unwrap();

        let base_client = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        let worktime_accounting_client = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/WorktimeAccountingService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        BlueantService {
            python: python,
            base_client,
            worktime_accounting_client,
        }
    }

    pub fn login(
        &self,
        username: &String,
        password: &String,
    ) -> Result<Session, serde_json::Error> {
        let zeep_module = self.python.import("zeep").unwrap();
        let json_module = self.python.import("json").unwrap();

        let locals = PyDict::new(self.python);
        locals.set_item("zeep", zeep_module).unwrap();
        locals.set_item("json", json_module).unwrap();
        locals.set_item("client", self.base_client).unwrap();
        locals.set_item("username", username).unwrap();
        locals.set_item("password", password).unwrap();

        let result: String = self.python
            .eval(
                "json.dumps(zeep.helpers.serialize_object(client.service.Login(username, password)))",
                None,
                Some(&locals),
            )
            .unwrap().extract().unwrap();
        let session: Session = serde_json::from_str(&result)?;

        Ok(session)
    }

    pub fn get_projects(&self, session: &Session) -> Result<Vec<Project>, Error> {
        let zeep_module = self.python.import("zeep").unwrap();
        let json_module = self.python.import("json").unwrap();

        let locals = PyDict::new(self.python);
        locals.set_item("zeep", zeep_module).unwrap();
        locals.set_item("json", json_module).unwrap();
        locals
            .set_item("client", self.worktime_accounting_client)
            .unwrap();
        locals.set_item("sessionID", &session.sessionID).unwrap();

        let result = self.python.eval(
            "json.dumps(zeep.helpers.serialize_object(client.service.getProjects(sessionID)), default=str)",
            None,
            Some(&locals),
        );

        match result {
            Ok(v) => {
                let extracted_value: PyResult<String> = v.extract();
                match extracted_value {
                    Ok(json) => Ok(deserealize(&json)?),
                    Err(python_err) => Err(Error::PythonErr),
                }
            }
            Err(e) => Err(Error::PythonErr),
        }
    }
}

fn deserealize<'a, T>(json: &'a str) -> Result<T, Error>
where
    T: serde::Deserialize<'a>,
{
    match serde_json::from_str(&json) {
        Ok(result) => Ok(result),
        Err(e) => {
            println!("{:?}", e);
            Err(Error::JsonError)
        }
    }
}
