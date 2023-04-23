#!python3
# arduino-serial.py - very basic CLI for sending newline-delimited serial commands to an arduino
# requires pyserial
import sys
import serial
import glob
import time

# It's going to attempt to connect to the first USB, so make sure there's only one plugged in
port = glob.glob('/dev/tty.usb*')[0]
arduino = serial.Serial(port=port, baudrate=9600, timeout=.1)

def write(x):
    arduino.write(bytes(x, 'ascii'))
    time.sleep(0.05)
    data = arduino.readline()
    return data

print("Connected to port " + port)
print("Waiting for arduino to be ready for input")
time.sleep(2)
print("Now accepting input. Enter a blank line to exit.")
while True:
    command = input('Enter a command: ')
    if command == '':
        sys.exit(0)

    write(command + '\n')
