// v1

// #[cxx_qt::bridge]
// mod my_object {
//
//     #[cxx_qt::qobject(qml_uri = "caesar", qml_version = "1.0")]
//     #[derive(Default)]
//     pub struct Hello {}
//
//     impl qobject::Hello {
//         #[qinvokable]
//         pub fn say_hello(&self) {
//             println!("Hello world!")
//         }
//     }
// }

//v2

#[cxx_qt::bridge]
mod my_object {

    unsafe extern "C++" {
        include!("cxx-qt-lib/qstring.h");
        type QString = cxx_qt_lib::QString;
    }

    #[cxx_qt::qobject(qml_uri = "caesar", qml_version = "1.0")]
    pub struct Rot {
        #[qproperty]
        plain: QString,
        #[qproperty]
        secret: QString,
    }

    impl Default for Rot {
        fn default() -> Self {
            Self {
                plain: QString::from(""),
                secret: QString::from(""),
            }
        }
    }

    impl qobject::Rot {
        #[qinvokable]
        pub fn encrypt(&self, plain: &QString) -> QString {
            let result = format!("{plain} is a secret");
            QString::from(&result)
        }
    }

    #[cxx_qt::qobject(qml_uri = "caesar", qml_version = "1.0")]
    #[derive(Default)]
    pub struct Hello {}

    impl qobject::Hello {
        #[qinvokable]
        pub fn say_hello(&self) {
            println!("Hello world!")
        }
    }
}
