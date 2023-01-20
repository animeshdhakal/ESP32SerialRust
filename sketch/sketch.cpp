#include <WiFi.h>
#include <Wire.h>

enum MessageType {
    BEGIN,
    MESSAGE,
    END
};

struct SerialMessageHeader {
    uint32_t msg_type;
    uint32_t msg_length;
};

void setup(){
    Serial.begin(115200);
    pinMode(LED_BUILTIN, OUTPUT);
}

void blinkLEDFor(int milli){
    int now = millis();
    while(millis() - now < milli){
        digitalWrite(LED_BUILTIN, HIGH);
        delay(100);
        digitalWrite(LED_BUILTIN, LOW);
        delay(100);
    }
}

void loop(){
    if(Serial.available()){
        SerialMessageHeader header;
        size_t read = Serial.read((uint8_t*)&header, sizeof(header));

        if(read == sizeof(header)) {
            if(header.msg_type == MessageType::BEGIN){
                header.msg_type = MessageType::MESSAGE;

                char message[] = "Hello World!";
                header.msg_length = sizeof(message);

                Serial.write((uint8_t*)&header, sizeof(header));
                Serial.write((uint8_t*)message, sizeof(message));
                    
                blinkLEDFor(3000);
            }
        }else{
            blinkLEDFor(2000);
        }

    }
}