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
  while (! Serial); // Wait until Serial is ready
  Serial.println("Ready to accept input");
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readString();

    Serial.println(input);
    
    char command = input[0];
    int duration = (input[1] - '0') * 1000;

    switch (command) {
      case 'f':
        forward();
        break;
      case 'b':
        backward();
        break;
      case 'l':
        left();
        break;
      case 'r':
        right();
        break;
      case 's':
        straight();
        break;
      default:
        break;
    }

    // TODO add error handling

    delay(duration);
    stop();
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
