# CHANGELOG

## v1.3.0

- Feat: add frame rate limit support. #6

## v1.2.2

- Fix: use the sync version gRPC call to make sure `HDD_Monitor/HDD_Manager` are properly destroyed.

## v1.2.1

- Enhance stability.

## v1.2.0

- Feat: `HDD_Manager.Refresh` will restart shremdup service.
- Enhance stability.

## v1.1.1

Fix: missing gRPC packages.

## v1.1.0

- Feat: add `HDD_Monitor.id`.
- Feat: add `HDD_Manager.primaryIndex/primaryInfo`.
- Feat: optimize `HDD_Monitor` destruction behaviour, call `Destroy(object)` will also destroy the monitor in the server.
- Optimize folder structure.

## v1.0.0

The initial release.
