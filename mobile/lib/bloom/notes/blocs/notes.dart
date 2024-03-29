import 'dart:async';

import 'package:bloom/bloom/kernel/blocs/bloc_provider.dart';
import 'package:bloom/bloom/notes/models/note.dart';

class NotesBloc extends BlocBase {
  NotesBloc();

  final StreamController<List<Note>> _notesController =
      StreamController<List<Note>>.broadcast();
  Stream<List<Note>> get notesOut => _notesController.stream;

  @override
  void dispose() {
    _notesController.close();
  }

  Future<void> getNotes() async {
    _notesController.sink.add(await Note.find());
  }

  Future<void> getArchive() async {
    _notesController.sink.add(await Note.findArchived());
  }
}

final NotesBloc notesBloc = NotesBloc();
