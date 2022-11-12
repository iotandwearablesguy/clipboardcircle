# calculations_of_a_circle

   **Calculations of a Circle Application_ App**
            _© 2022 Terrance O'Shea_
            _terry.oshea@me.com_
### Intro:

Example of GTK4 Package Using Calculation of a Circle and Text Augmenting."

### Acknowledgements:

* The Gui's are made with gtk4

* The application reads from

```rust
let from_entry = gtk::Entry::builder()
    .placeholder_text("Radius")
    .build();
text_container.append(&from_entry);
let btn = gtk::Button::with_label("Enter");
btn.connect_clicked(clone!(@weak clipboard, @weak from_entry => move |_btn| {
    let text = from_entry.text();
    let radius: f32 = text.trim().parse::<f32>().unwrap();
    let circum_circle: f32 = radius * 2. * 3.1415;
    let circumanswer = circum_circle.to_string();
    clipboard.set_text(&circumanswer)
  }));
```

 ## LICENSE

_O'Shea-Dishongh No Harm License© 2022 Terrance O'Shea & Katherine Dishongh_

<http://github.com/terryo-iotandwearablesguy>

Subject to the following conditions:

 1. That the below copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

 2. That any use of the Software under this license never be used for harmful purposes, permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction,including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to
 do so subject to the aforementioned restrictions.

"Harmful purposes" as used herein means to not harm destroy or otherwise restrict humans, animals, or the environment. "Harmful purposes" includes, inter alia, the creation of weaponry, the guidance of weapons, limitation of food and services, damaging, contaminating or otherwise hazardous to the environmnet in any form, and/or restricting the freedom or movement of people.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE




