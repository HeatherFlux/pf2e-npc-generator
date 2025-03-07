use stylist::{style, Style};

pub fn get_styles() -> Style {
    style!(
        r#"
        .pf2e-page {
            max-width: 1000px;
            margin: 40px auto;
            padding: 40px;
            background-color: #f4e4bc;
            position: relative;
            border: 2px solid #1c2c1f;
        }

        .pf2e-page::before {
            content: '';
            position: absolute;
            top: -20px;
            bottom: -20px;
            left: 10px;
            right: 10px;
            border-left: 2px solid #1c2c1f;
            border-right: 2px solid #1c2c1f;
        }

        .pf2e-page::after {
            content: '';
            position: absolute;
            top: 10px;
            bottom: 10px;
            left: -20px;
            right: -20px;
            border-top: 2px solid #1c2c1f;
            border-bottom: 2px solid #1c2c1f;
        }

        .form-container {
            background-color: #fff9e6;
            padding: 20px;
            margin-bottom: 20px;
            border: 2px solid #1c2c1f;
            position: relative;
        }

        .form-container::before {
            content: '';
            position: absolute;
            top: -10px;
            bottom: -10px;
            left: 5px;
            right: 5px;
            border-left: 2px solid #1c2c1f;
            border-right: 2px solid #1c2c1f;
        }

        .form-container::after {
            content: '';
            position: absolute;
            top: 5px;
            bottom: 5px;
            left: -10px;
            right: -10px;
            border-top: 2px solid #1c2c1f;
            border-bottom: 2px solid #1c2c1f;
        }

        h1 {
            color: #1c2c1f;
            font-size: 2.5em;
            text-align: left;
            font-family: 'IM Fell English', serif;
            margin-bottom: 30px;
            position: relative;
            padding-bottom: 15px;
        }

        h1::after {
            content: '';
            position: absolute;
            bottom: 0;
            left: 0;
            right: 0;
            height: 10px;
            background-image: url("data:image/svg+xml,%3Csvg xmlns='http://www.w3.org/2000/svg' width='100' height='10' viewBox='0 0 100 10'%3E%3Cpath fill='none' stroke='%231c2c1f' stroke-width='1' d='M0,5 C25,0 75,10 100,5 M20,5 C40,0 60,10 80,5'/%3E%3C/svg%3E");
            background-repeat: repeat-x;
            background-size: 100px 10px;
            pointer-events: none;
        }

        .form-group {
            display: flex;
            align-items: center;
            gap: 1rem;
            margin-bottom: 1rem;
            position: relative;
            z-index: 1;
        }

        label {
            min-width: 100px;
            font-family: 'IM Fell English', serif;
            font-size: 1.2em;
            color: #1c2c1f;
        }

        input, select {
            padding: 8px;
            border: 1px solid #1c2c1f;
            background-color: #fff;
            font-family: 'IM Fell English', serif;
            flex: 1;
        }

        button {
            padding: 10px 20px;
            background-color: #1c2c1f;
            color: #f4e4bc;
            border: none;
            cursor: pointer;
            font-family: 'IM Fell English', serif;
            font-size: 1.1em;
            position: relative;
            z-index: 1;
        }

        .statblock {
            background-color: #fff9e6;
            padding: 20px;
            margin-top: 20px;
            border: 2px solid #1c2c1f;
            position: relative;
        }

        .statblock::before {
            content: '';
            position: absolute;
            top: -10px;
            bottom: -10px;
            left: 5px;
            right: 5px;
            border-left: 2px solid #1c2c1f;
            border-right: 2px solid #1c2c1f;
        }

        .statblock::after {
            content: '';
            position: absolute;
            top: 5px;
            bottom: 5px;
            left: -10px;
            right: -10px;
            border-top: 2px solid #1c2c1f;
            border-bottom: 2px solid #1c2c1f;
        }

        .stat-group {
            display: grid;
            grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
            gap: 10px;
            margin: 10px 0;
            position: relative;
            z-index: 1;
        }

        .stat-item {
            background-color: #f4e4bc;
            padding: 8px 12px;
            text-align: center;
            border: 1px solid #1c2c1f;
        }
    "#
    ).unwrap()
} 