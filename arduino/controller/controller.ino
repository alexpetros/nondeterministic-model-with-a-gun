/*
  RC Control

  Prototype implementation of RC control for the aduino, based on serial commands.

  The circuit:
  - LED attached from pin 2 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 3 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 5 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 6 through 220 ohm resistor, and to transitor via 1k ohm resistor
*/

const int forwardPin = 2;
const int backwardPin = 3;
const int leftPin = 5;
const int rightPin = 6;

void setup() {
  pinMode(forwardPin, OUTPUT);
  pinMode(backwardPin, OUTPUT);
  pinMode(leftPin, OUTPUT);
  pinMode(rightPin, OUTPUT);
  Serial.begin(9600);
  stop();
}

void loop() {
  if (Serial.available()) {
    char ch = Serial.read();
    switch (ch) {
      case 'i':
        forward();
        break;
      case 'k':
        backward();
        break;
      case 'j':
        left();
        break;
      case 'l':
        right();
        break;
      case ' ':
        straight();
        break;
      case 's':
        stop();
        break;
      default:
        break;
    }
  }
}

void stop() {
  digitalWrite(forwardPin, LOW);
  digitalWrite(backwardPin, LOW);
  digitalWrite(leftPin, LOW);
  digitalWrite(rightPin, LOW);
}

void forward() {
  Serial.println("Moving car forward");
  digitalWrite(forwardPin, HIGH);
  digitalWrite(backwardPin, LOW);
}

void backward() {
  Serial.println("Moving car backward");
  digitalWrite(forwardPin, LOW);
  digitalWrite(backwardPin, HIGH);
}

void left() {
  Serial.println("Moving wheels left");
  digitalWrite(leftPin, HIGH);
  digitalWrite(rightPin, LOW);
}

void right() {
  Serial.println("Moving wheels right");
  digitalWrite(leftPin, LOW);
  digitalWrite(rightPin, HIGH);
}

void straight() {
  Serial.println("Moving wheels straight");
  digitalWrite(leftPin, LOW);
  digitalWrite(rightPin, LOW);
}
