use pyo3::{GILGuard, ObjectProtocol, PyDict, PyObjectRef, PyResult, Python};

pub struct BaseService<'a> {
    python: Python<'a>,
    client: &'a PyObjectRef,
}

impl<'a> BaseService<'a> {
    pub fn get_python_gil() -> GILGuard {
        Python::acquire_gil()
    }

    pub fn new(gil: &'a GILGuard) -> Self {
        let python = gil.python();
        let zeep_module = python.import("zeep").unwrap();
        let locals = PyDict::new(python);
        locals.set_item("zeep", zeep_module).unwrap();

        let client = python.eval(
            "zeep.Client('https://blueant.sinnerschrader.com/blueant/services/BaseService?wsdl')",
            None,
            Some(&locals),
        ).unwrap();

        BaseService {
            python: python,
            client,
        }
    }

    pub fn login(&self, username: &String, password: &String) {
        let zeep_module = self.python.import("zeep").unwrap();

        let locals = PyDict::new(self.python);
        locals.set_item("zeep", zeep_module).unwrap();
        locals.set_item("client", self.client).unwrap();
        locals.set_item("username", username).unwrap();
        locals.set_item("password", password).unwrap();

        let result = self.python
            .eval(
                "dict(zeep.helpers.serialize_object(client.service.Login(username, password)))",
                None,
                Some(&locals),
            )
            .unwrap();

        println!("{:?}", result);
    }
}
