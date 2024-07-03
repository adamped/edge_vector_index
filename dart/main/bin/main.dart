import 'package:main/main.dart' as lib;

void main(List<String> arguments) {

  var store = lib.Store();

  store.addToIndex([0.1, 0.2], "1");
  store.addToIndex([0.3, 0.4], "2");

  print(store.findClosestMatch([0.3, 0.4]));

  store.dispose();

  print('Finished');
}
