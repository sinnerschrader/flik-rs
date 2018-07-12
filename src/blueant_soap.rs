use pyo3::{GILGuard, ObjectProtocol, PyDict, PyObjectRef, PyResult, Python};
use serde_json;

#[derive(Serialize, Deserialize, Debug)]
pub struct Session {
    personID: i32,
    sessionID: String,
}

pub struct BlueantService<'a> {
    python: Python<'a>,
    baseClient: &'a PyObjectRef,
    worktimeAccountingClient: &'a PyObjectRef,
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

        let baseClient = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        let worktimeAccountingClient = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/WorktimeAccountingService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        BlueantService {
            python: python,
            baseClient,
            worktimeAccountingClient,
        }
    }

    pub fn login(&self, username: &String, password: &String) -> Result<Session, serde_json::Error> {
        let zeep_module = self.python.import("zeep").unwrap();
        let json_module = self.python.import("json").unwrap();

        let locals = PyDict::new(self.python);
        locals.set_item("zeep", zeep_module).unwrap();
        locals.set_item("json", json_module).unwrap();
        locals.set_item("client", self.baseClient).unwrap();
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
        println!("{:?}", session);
        Ok(session)
    }

}
