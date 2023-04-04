/*
  RC Control

  Prototype implementation of RC control for the aduino, based on serial commands.
  
  Turns on and off two LEDs connected to pins 3 and 4 based on serial input.

  The circuit:
  - LED attached from pin 3 to ground through 220 ohm resistor
  - LED attached from pin 4 to ground through 220 ohm resistor
*/

const int forwardPin = 4;
const int backwardPin = 3;

void setup() {
  pinMode(forwardPin, OUTPUT);
  pinMode(backwardPin, OUTPUT);
  Serial.begin(9600);
  clear();
}

void loop() {
  if (Serial.available()) {
    char ch = Serial.read();
    switch (ch) {
      case 'f': 
        toggle(forwardPin);
        Serial.println("Toggled forward LED");
        break;
      case 'b':
        toggle(backwardPin);
        Serial.println("Toggled backward LED");
        break;
      case 'x': 
        clear();
        Serial.println("Cleared");
        break;
      default:
        break;
    }
  }
}

void clear() {
  digitalWrite(forwardPin, LOW);
  digitalWrite(backwardPin, LOW);
}

void toggle(int pin) {
  digitalWrite(pin, !digitalRead(pin));
}