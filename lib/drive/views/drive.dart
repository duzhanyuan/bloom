import 'package:bloom/kernel/widgets/drawer.dart';
import 'package:flutter/material.dart';

class DriveView extends StatefulWidget {
  const DriveView();

  @override
  _DriveState createState() => _DriveState();
}

class _DriveState extends State<DriveView> {
  @override
  Widget build(BuildContext context) {
    return Scaffold(
      drawer: const BlmDrawer(),
      appBar: AppBar(
        title: const Text('Drive'),
      ),
      body: Center(child: const Text('Drive')),
    );
  }
}
