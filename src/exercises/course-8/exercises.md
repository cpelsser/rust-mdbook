# Exercises

Time to put it together! In this exercise, you'll combine LEDs and buttons
to create an interactive LED pattern controller.

## Exercise: LED Pattern Controller

[LED Pattern Controller](led-pattern.md)

Build a program that:

1. Displays patterns on the LED matrix
2. Uses buttons to control the display

This exercise practices:

- GPIO output (LEDs)
- GPIO input (buttons)
- State management without heap allocation
- Embedded Rust program structure

<details>

**Key points:**

- This exercise combines input (buttons) and output (LEDs) in one program
- State management without heap allocation is a key embedded skill
- Row scanning demonstrates how real LED matrices work

**Tips for students:**

- Start simple - get one pattern working first
- Use arrays to store patterns (they're stack-allocated)
- Test button logic separately before combining with LEDs

**Common student questions:**

- *"Why does my display flicker?"* - The row scanning delay might be too long. Try
  reducing the delay between rows.
- *"The button seems to register multiple presses"* - You need edge detection (check
  for transition from unpressed to pressed), not just level detection.
- *"Can I use a loop instead of repeating code for each row?"* - Yes, but the borrow
  checker makes it tricky. The Display API handles this for you.

</details>
