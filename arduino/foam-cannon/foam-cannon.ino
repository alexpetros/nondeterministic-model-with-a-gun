/*
  RC Control

  Prototype implementation of RC control for the aduino, based on serial commands.

  The circuit:
  - LED attached from pin 2 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 3 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 5 through 220 ohm resistor, and to transitor via 1k ohm resistor
  - LED attached from pin 6 through 220 ohm resistor, and to transitor via 1k ohm resistor
*/

const int shootPin = 2;
const int leftPin = 5;
const int rightPin = 6;
const int connectionTestPin = 3;

void setup() {
  pinMode(shootPin, OUTPUT);
  pinMode(leftPin, OUTPUT);
  pinMode(rightPin, OUTPUT);
  pinMode(connectionTestPin, OUTPUT);
  Serial.begin(9600);
  stop();
  while (! Serial); // Wait until Serial is ready
  Serial.println("Ready to accept input");
}

void loop() {
  if (Serial.available()) {
    String input = Serial.readStringUntil('\n');

    Serial.println(input);

    char command = input[0];
    int duration_ms = (input[1] - '0') * 1000;

    switch (command) {
      case 's':
        shoot();
        break;
      case 'l':
        left();
        break;
      case 'r':
        right();
        break;
      case 'c':
        enableConnectionTest();
        break;
      default:
        break;
    }

    // TODO add error handling
    delay(duration_ms / 2);
    stop();
  }
}

void stop() {
  digitalWrite(shootPin, LOW);
  digitalWrite(leftPin, LOW);
  digitalWrite(rightPin, LOW);
  digitalWrite(connectionTestPin, LOW);
}

void shoot() {
  Serial.println("Shooting");
  digitalWrite(shootPin, HIGH);
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

void enableConnectionTest() {
  Serial.println("Moving wheels right");
  digitalWrite(connectionTestPin, HIGH);
}
