short steppers[][5] = {
  //{STEP, DIR, EN, +5V, STEPS/REV}
  {30, 32, 34, 36, 200}, //Left
  {31, 33, 35, 37, 200}, //Back
  {38, 40, 42, 44, 200}, //Front
  {39, 41, 43, 45, 200}, //Top
  {46, 48, 50, 52, 200}, //Bottom
  {47, 49, 51, 53, 200}  //Right
};

char commands[32];

int steps_per_quarter_turn = 100;

char get_nybble(char* arr, int i) {
  if (i % 2 == 0)
    return arr[i] >> 4;
  else
    return arr[i] % 16;
}

char get_cmd_stepper_index(char cmd) {
  return cmd % 8;
}

char get_cmd_direction(char cmd) {
  return cmd >> 3;
}

void init_stepper(short* stepper) {
  pinMode(stepper[0], OUTPUT);
  pinMode(stepper[1], OUTPUT);
  pinMode(stepper[2], OUTPUT);
  pinMode(stepper[3], OUTPUT);

  digitalWrite(stepper[2], HIGH);
  digitalWrite(stepper[3], HIGH);
}

void enable_stepper(short* stepper) {
  digitalWrite(stepper[2], HIGH);
}

void disable_stepper(short* stepper) {
  digitalWrite(stepper[2], LOW);
}

void step_stepper(short* stepper, int steps, int d) {
  digitalWrite(stepper[1], ((steps < 0) ? HIGH : LOW));
  for (int i = 0; i < abs(steps); i++) {
    digitalWrite(stepper[0], HIGH);
    delayMicroseconds(d);
    digitalWrite(stepper[0], LOW);
    delayMicroseconds(d);
  }
}

void simul_step_steppers(short* stepper1, int steps1, short* stepper2, int steps2, int d) {
  digitalWrite(stepper1[1], ((steps1 < 0) ? HIGH : LOW));
  digitalWrite(stepper2[1], ((steps2 < 0) ? HIGH : LOW));
  for (int i = 0; i < max(abs(steps1), abs(steps2)); i++) {
    if (i < steps1) digitalWrite(stepper1[0], HIGH);
    if (i < steps2) digitalWrite(stepper2[0], HIGH);
    delayMicroseconds(d);
    if (i < steps1) digitalWrite(stepper1[0], LOW);
    if (i < steps2) digitalWrite(stepper2[0], LOW);
    delayMicroseconds(d);
  }
}

void setup() {
  for (int i = 0; i < 6; i++) {
    init_stepper(steppers[i]);
  }
  Serial.begin(9600);
}

void loop() {
  int n;
  if (n = Serial.available()) {
    char buf[2];

    Serial.readBytes(buf, 2);
    char command = buf[0];
    char dir = buf[1];

    Serial.print(command);
    Serial.println(dir);

    int stepper_i = -1;
    switch (command) {
      case 'f'://front
        stepper_i = 2;
        break;
      case 't'://top
        stepper_i = 3;
        break;
      case 'l'://left
        stepper_i = 0;
        break;
      case 'r'://right
        stepper_i = 5;
        break;
      case 'b'://back
        stepper_i = 1;
        break;
      case 'B'://bottom
        stepper_i = 4;
        break;
      default:
        break;
    }

    if (stepper_i != -1) {
      if (dir == '+' || dir == '-')
        step_stepper(steppers[stepper_i], ((dir == '+') ? 1 : -1) * 100, 275);
      else if (dir == 'd')
        disable_stepper(steppers[stepper_i]);
      else if (dir == 'e')
        enable_stepper(steppers[stepper_i]);
    }
  }
}
