/*
  RC Control

  Prototype implementation of RC control for the aduino, based on serial commands.
*/

const int shootPin = 2;
const int leftPin = 3;
const int rightPin = 4;
const int raisePin = 5;
const int connectionTestPin = 13;

void setup() {
  pinMode(shootPin, OUTPUT);
  pinMode(leftPin, OUTPUT);
  pinMode(rightPin, OUTPUT);
  pinMode(raisePin, OUTPUT);
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
      case 'u':
        raise();
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
  digitalWrite(raisePin, LOW);
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

void raise() {
  Serial.println("Raising up");
  digitalWrite(raisePin, HIGH);
}

void enableConnectionTest() {
  Serial.println("Running connection test");
  digitalWrite(connectionTestPin, HIGH);
}
