package com.orvpass.app;

import android.app.Activity;
import android.os.Bundle;
import android.widget.TextView;

public class MainActivity extends Activity {

    @Override
    protected void onCreate(Bundle state) {
        super.onCreate(state);

        TextView view = new TextView(this);
        view.setText(
            "Orvpass\n\nSecure Password Manager\nv3.1"
        );

        view.setTextSize(24);

        setContentView(view);
    }
}
