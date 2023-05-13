import QtQuick.Controls 2.12
import QtQuick.Window 2.12

// This must match the qml_uri and qml_version
// specified with the #[cxx_qt::qobject] macro in Rust.
import caesar 1.0

Window {
    title: qsTr("Caesar")
    visible: true
    height: 480
    width: 640
    color: "#e4af79"

    Hello {
      id: hello
    }

    Rot {
        id: rot
        plain: ""
        secret: ""
    }

    Column {
        anchors.horizontalCenter: parent.horizontalCenter
        anchors.verticalCenter: parent.verticalCenter
        /* space between widget */
        spacing: 10

        Button {
            text: "Say Hello!"
            onClicked: hello.sayHello()
        }

        Label {
          text: "Keep your secret safe 🔒"
          font.bold: true
        } 
        TextArea {
          placeholderText: qsTr("me@caesar.tld")
          text: rot.plain
          onTextChanged: rot.plain = text

          background: Rectangle {
            implicitWidth: 400
            implicitHeight: 50
            radius: 3
            color:  "#e2e8f0"
            border.color:  "#21be2b"
          }
        }
       Button {
         text: "Encrypt!"
         onClicked: rot.secret = rot.encrypt(rot.plain)
        }

        Label {
          text: rot.secret
        }
    }
}
