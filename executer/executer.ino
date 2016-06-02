#include<stdarg.h>

class Stepper {
  private:
    short STEP, DIR, EN, PWR;

  public:

    Stepper(short _STEP, short _DIR, short _EN, short _PWR) {
      STEP = _STEP;
      DIR  = _DIR;
      EN   = _EN;
      PWR  = _PWR;
    }

    void init() {
      pinMode(STEP, OUTPUT);
      pinMode(DIR, OUTPUT);
      pinMode(EN, OUTPUT);
      pinMode(PWR, OUTPUT);

      digitalWrite(EN, HIGH);
      digitalWrite(PWR, HIGH);
    }

    void enable() {
      digitalWrite(EN, HIGH);
    }

    void disable() {
      digitalWrite(EN, LOW);
    }

    void step(int steps, int d) {
      digitalWrite(DIR, ((steps < 0) ? HIGH : LOW));
      for (int i = 0; i < abs(steps); i++) {
        digitalWrite(STEP, HIGH);
        delayMicroseconds(d);
        digitalWrite(STEP, LOW);
        delayMicroseconds(d);
      }
    }


    static void simul_step_steppers(Stepper s1, int steps1, Stepper s2, int steps2, int d) {
      digitalWrite(s1.DIR, ((steps1 < 0) ? HIGH : LOW));
      digitalWrite(s2.DIR, ((steps2 < 0) ? HIGH : LOW));
      for (int i = 0; i < max(abs(steps1), abs(steps2)); i++) {
        if (i < steps1) digitalWrite(s1.STEP, HIGH);
        if (i < steps2) digitalWrite(s2.STEP, HIGH);
        delayMicroseconds(d);
        if (i < steps1) digitalWrite(s1.STEP, LOW);
        if (i < steps2) digitalWrite(s2.STEP, LOW);
        delayMicroseconds(d);
      }
    }
};

char get_nybble(char* arr, int i) {
  if (i % 2 == 0)
    return arr[i / 2] >> 4;
  else
    return arr[i / 2] % 16;
}

char get_cmd_stepper_index(char cmd) {
  return cmd % 8;
}

char get_cmd_direction(char cmd) {
  return (cmd >> 3) % 2;
}

short stepper_index(char c) {
  switch (c) {
    case 'u': case 'U'://top
      return 0;
      break;
    case 'r': case 'R'://right
      return 1;
      break;
    case 'f': case 'F'://front
      return 2;
      break;
    case 'd': case 'D'://bottom
      return 3;
      break;
    case 'l': case 'L'://left
      return 4;
      break;
    case 'b': case 'B'://back
      return 5;
      break;
    default:
      return -1;
      break;
  }
}

bool debug = false;
int steps_per_quarter_turn = 100;
int micros_between_steps = 250;
int millis_between_moves = 10;

Stepper steppers[6] = {
  //{STEP, DIR, EN, +5V, STEPS/REV}
  Stepper(39, 41, 43, 45), //Up    => 0
  Stepper(30, 32, 34, 36), //Right => 1
  Stepper(38, 40, 42, 44), //Front => 2
  Stepper(46, 48, 50, 52), //Down  => 3
  Stepper(47, 49, 51, 53), //Left  => 4
  Stepper(31, 33, 35, 37)  //Back  => 5
};

void setup() {
  for (int i = 0; i < 6; i++) {
    steppers[i].init();
  }
  Serial.begin(9600);
}

void loop() {
  char buf[64];
  if (Serial.available()) {
    Serial.readBytes(buf, 1);
    char command = buf[0];
    if (command == 's') {
      //receiving a sequence of moves from the serial port.
      short n = 0;
      Serial.readBytes((char*)(&n), 1); //big endian encoding

      if ((n + 1) / 2 > 64) {
        Serial.println("Incoming buffer too long!");
      } else {
        //read and execute the manuever
        Serial.readBytes(buf, n);

        for (int i = 0; i < n; i++) {
          char cmd = buf[i];

          steppers[get_cmd_stepper_index(cmd)].step((get_cmd_direction(cmd) ? 1 : -1)*steps_per_quarter_turn , micros_between_steps);
          delay(millis_between_moves);
        }
      }
    } else if (command == 'O') {
      Serial.println(debug ? "Debugging off." : "Debugging on.");
      debug = !debug;
    } else {
      Serial.readBytes(buf, 1);
      char dir = buf[0];

      Serial.print(command);
      Serial.println(dir);

      int stepper_i = stepper_index(command);

      if (stepper_i != -1) {
        if (dir == '+' || dir == '-')
          steppers[stepper_i].step(((dir == '+') ? 1 : -1) * ((command & 32)?2:100), micros_between_steps);
        else if (dir == 'd')
          steppers[stepper_i].disable();
        else if (dir == 'e')
          steppers[stepper_i].enable();
      }
    }
  }
}
