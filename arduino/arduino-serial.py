# arduino-serial.py - very basic CLI for sending newline-delimited serial commands to an arduino
# requires pyserial
import sys
import serial
import time

# TODO make this dynamic
arduino = serial.Serial(port='/dev/cu.usbmodem1112401', baudrate=9600, timeout=.1)

def write(x):
    arduino.write(bytes(x, 'ascii'))
    time.sleep(0.05)
    data = arduino.readline()
    return data

print("Now accepting input. Enter a blank line to exit.")
while True:
    command = input('Enter a command: ')
    if command == '':
        sys.exit(0)

    write(command + '\n')
