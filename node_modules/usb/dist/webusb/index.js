"use strict";
var __awaiter = (this && this.__awaiter) || function (thisArg, _arguments, P, generator) {
    function adopt(value) { return value instanceof P ? value : new P(function (resolve) { resolve(value); }); }
    return new (P || (P = Promise))(function (resolve, reject) {
        function fulfilled(value) { try { step(generator.next(value)); } catch (e) { reject(e); } }
        function rejected(value) { try { step(generator["throw"](value)); } catch (e) { reject(e); } }
        function step(result) { result.done ? resolve(result.value) : adopt(result.value).then(fulfilled, rejected); }
        step((generator = generator.apply(thisArg, _arguments || [])).next());
    });
};
var __generator = (this && this.__generator) || function (thisArg, body) {
    var _ = { label: 0, sent: function() { if (t[0] & 1) throw t[1]; return t[1]; }, trys: [], ops: [] }, f, y, t, g;
    return g = { next: verb(0), "throw": verb(1), "return": verb(2) }, typeof Symbol === "function" && (g[Symbol.iterator] = function() { return this; }), g;
    function verb(n) { return function (v) { return step([n, v]); }; }
    function step(op) {
        if (f) throw new TypeError("Generator is already executing.");
        while (_) try {
            if (f = 1, y && (t = op[0] & 2 ? y["return"] : op[0] ? y["throw"] || ((t = y["return"]) && t.call(y), 0) : y.next) && !(t = t.call(y, op[1])).done) return t;
            if (y = 0, t) op = [op[0] & 2, t.value];
            switch (op[0]) {
                case 0: case 1: t = op; break;
                case 4: _.label++; return { value: op[1], done: false };
                case 5: _.label++; y = op[1]; op = [0]; continue;
                case 7: op = _.ops.pop(); _.trys.pop(); continue;
                default:
                    if (!(t = _.trys, t = t.length > 0 && t[t.length - 1]) && (op[0] === 6 || op[0] === 2)) { _ = 0; continue; }
                    if (op[0] === 3 && (!t || (op[1] > t[0] && op[1] < t[3]))) { _.label = op[1]; break; }
                    if (op[0] === 6 && _.label < t[1]) { _.label = t[1]; t = op; break; }
                    if (t && _.label < t[2]) { _.label = t[2]; _.ops.push(op); break; }
                    if (t[2]) _.ops.pop();
                    _.trys.pop(); continue;
            }
            op = body.call(thisArg, _);
        } catch (e) { op = [6, e]; y = 0; } finally { f = t = 0; }
        if (op[0] & 5) throw op[1]; return { value: op[0] ? op[1] : void 0, done: true };
    }
};
var __values = (this && this.__values) || function(o) {
    var s = typeof Symbol === "function" && Symbol.iterator, m = s && o[s], i = 0;
    if (m) return m.call(o);
    if (o && typeof o.length === "number") return {
        next: function () {
            if (o && i >= o.length) o = void 0;
            return { value: o && o[i++], done: !o };
        }
    };
    throw new TypeError(s ? "Object is not iterable." : "Symbol.iterator is not defined.");
};
Object.defineProperty(exports, "__esModule", { value: true });
exports.WebUSB = exports.getWebUsb = void 0;
var usb = require("../usb");
var events_1 = require("events");
var webusb_device_1 = require("./webusb-device");
/**
 * Convenience method to get the WebUSB interface available
 */
var getWebUsb = function () {
    if (navigator && navigator.usb) {
        return navigator.usb;
    }
    return new WebUSB();
};
exports.getWebUsb = getWebUsb;
var WebUSB = /** @class */ (function () {
    function WebUSB(options) {
        var _this = this;
        if (options === void 0) { options = {}; }
        this.options = options;
        this.emitter = new events_1.EventEmitter();
        this.knownDevices = new Map();
        this.allowedDevices = options.allowedDevices || [];
        var deviceConnectCallback = function (device) { return __awaiter(_this, void 0, void 0, function () {
            var webDevice, deviceId, event_1;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0: return [4 /*yield*/, webusb_device_1.WebUSBDevice.createInstance(device)];
                    case 1:
                        webDevice = _a.sent();
                        // When connected, emit an event if it is an allowed device
                        if (webDevice && this.isAllowedDevice(webDevice)) {
                            deviceId = this.getDeviceId(device);
                            if (deviceId) {
                                this.knownDevices.set(deviceId, webDevice);
                            }
                            event_1 = {
                                type: 'connect',
                                device: webDevice
                            };
                            this.emitter.emit('connect', event_1);
                        }
                        return [2 /*return*/];
                }
            });
        }); };
        var deviceDisconnectCallback = function (device) { return __awaiter(_this, void 0, void 0, function () {
            var deviceId, webDevice, event_2;
            return __generator(this, function (_a) {
                deviceId = this.getDeviceId(device);
                // When disconnected, emit an event if the device was a known allowed device
                if (deviceId !== undefined && this.knownDevices.has(deviceId)) {
                    webDevice = this.knownDevices.get(deviceId);
                    if (webDevice && this.isAllowedDevice(webDevice)) {
                        event_2 = {
                            type: 'disconnect',
                            device: webDevice
                        };
                        this.emitter.emit('disconnect', event_2);
                    }
                }
                return [2 /*return*/];
            });
        }); };
        this.emitter.on('newListener', function (event) {
            var listenerCount = _this.emitter.listenerCount(event);
            if (listenerCount !== 0) {
                return;
            }
            if (event === 'connect') {
                usb.addListener('attach', deviceConnectCallback);
            }
            else if (event === 'disconnect') {
                usb.addListener('detach', deviceDisconnectCallback);
            }
        });
        this.emitter.on('removeListener', function (event) {
            var listenerCount = _this.emitter.listenerCount(event);
            if (listenerCount !== 0) {
                return;
            }
            if (event === 'connect') {
                usb.removeListener('attach', deviceConnectCallback);
            }
            else if (event === 'disconnect') {
                usb.removeListener('detach', deviceDisconnectCallback);
            }
        });
    }
    Object.defineProperty(WebUSB.prototype, "onconnect", {
        set: function (fn) {
            if (this._onconnect) {
                this.removeEventListener('connect', this._onconnect);
                this._onconnect = undefined;
            }
            if (fn) {
                this._onconnect = fn;
                this.addEventListener('connect', this._onconnect);
            }
        },
        enumerable: false,
        configurable: true
    });
    Object.defineProperty(WebUSB.prototype, "ondisconnect", {
        set: function (fn) {
            if (this._ondisconnect) {
                this.removeEventListener('disconnect', this._ondisconnect);
                this._ondisconnect = undefined;
            }
            if (fn) {
                this._ondisconnect = fn;
                this.addEventListener('disconnect', this._ondisconnect);
            }
        },
        enumerable: false,
        configurable: true
    });
    WebUSB.prototype.addEventListener = function (type, listener) {
        this.emitter.addListener(type, listener);
    };
    WebUSB.prototype.removeEventListener = function (type, callback) {
        this.emitter.removeListener(type, callback);
    };
    WebUSB.prototype.dispatchEvent = function (_event) {
        // Don't dispatch from here
        return false;
    };
    /**
     * Requests a single Web USB device
     * @param options The options to use when scanning
     * @returns Promise containing the selected device
     */
    WebUSB.prototype.requestDevice = function (options) {
        return __awaiter(this, void 0, void 0, function () {
            var devices, device, _a, error_1;
            var _this = this;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        // Must have options
                        if (!options) {
                            throw new TypeError('requestDevice error: 1 argument required, but only 0 present');
                        }
                        // Options must be an object
                        if (options.constructor !== {}.constructor) {
                            throw new TypeError('requestDevice error: parameter 1 (options) is not an object');
                        }
                        // Must have a filter
                        if (!options.filters) {
                            throw new TypeError('requestDevice error: required member filters is undefined');
                        }
                        // Filter must be an array
                        if (options.filters.constructor !== [].constructor) {
                            throw new TypeError('requestDevice error: the provided value cannot be converted to a sequence');
                        }
                        // Check filters
                        options.filters.forEach(function (filter) {
                            // Protocol & Subclass
                            if (filter.protocolCode && !filter.subclassCode) {
                                throw new TypeError('requestDevice error: subclass code is required');
                            }
                            // Subclass & Class
                            if (filter.subclassCode && !filter.classCode) {
                                throw new TypeError('requestDevice error: class code is required');
                            }
                        });
                        return [4 /*yield*/, this.loadDevices(options.filters)];
                    case 1:
                        devices = _b.sent();
                        devices = devices.filter(function (device) { return _this.filterDevice(options, device); });
                        if (devices.length === 0) {
                            throw new Error('requestDevice error: no devices found');
                        }
                        _b.label = 2;
                    case 2:
                        _b.trys.push([2, 6, , 7]);
                        if (!this.options.devicesFound) return [3 /*break*/, 4];
                        return [4 /*yield*/, this.options.devicesFound(devices)];
                    case 3:
                        _a = _b.sent();
                        return [3 /*break*/, 5];
                    case 4:
                        _a = devices[0];
                        _b.label = 5;
                    case 5:
                        device = _a;
                        if (!device) {
                            throw new Error('selected device not found');
                        }
                        if (!this.isAllowedDevice(device)) {
                            this.allowedDevices.push({
                                vendorId: device.vendorId,
                                productId: device.productId,
                                serialNumber: device.serialNumber
                            });
                        }
                        return [2 /*return*/, device];
                    case 6:
                        error_1 = _b.sent();
                        throw new Error("requestDevice error: " + error_1);
                    case 7: return [2 /*return*/];
                }
            });
        });
    };
    /**
     * Gets all allowed Web USB devices which are connected
     * @returns Promise containing an array of devices
     */
    WebUSB.prototype.getDevices = function () {
        return __awaiter(this, void 0, void 0, function () {
            var preFilters, devices;
            var _this = this;
            return __generator(this, function (_a) {
                switch (_a.label) {
                    case 0:
                        if (!this.options.allowAllDevices) {
                            // Create pre-filters
                            preFilters = this.allowedDevices.map(function (device) { return ({
                                vendorId: device.vendorId || undefined,
                                productId: device.productId || undefined,
                                serialNumber: device.serialNumber || undefined
                            }); });
                        }
                        return [4 /*yield*/, this.loadDevices(preFilters)];
                    case 1:
                        devices = _a.sent();
                        return [2 /*return*/, devices.filter(function (device) { return _this.isAllowedDevice(device); })];
                }
            });
        });
    };
    WebUSB.prototype.loadDevices = function (preFilters) {
        return __awaiter(this, void 0, void 0, function () {
            var devices, webDevices, devices_1, devices_1_1, device, webDevice, deviceId, e_1_1;
            var e_1, _a;
            return __generator(this, function (_b) {
                switch (_b.label) {
                    case 0:
                        devices = usb.getDeviceList();
                        // Pre-filter devices
                        devices = this.preFilterDevices(devices, preFilters);
                        webDevices = [];
                        _b.label = 1;
                    case 1:
                        _b.trys.push([1, 6, 7, 8]);
                        devices_1 = __values(devices), devices_1_1 = devices_1.next();
                        _b.label = 2;
                    case 2:
                        if (!!devices_1_1.done) return [3 /*break*/, 5];
                        device = devices_1_1.value;
                        if (this.options.deviceTimeout) {
                            device.timeout = this.options.deviceTimeout;
                        }
                        return [4 /*yield*/, webusb_device_1.WebUSBDevice.createInstance(device)];
                    case 3:
                        webDevice = _b.sent();
                        if (webDevice) {
                            webDevices.push(webDevice);
                            deviceId = this.getDeviceId(device);
                            if (deviceId) {
                                this.knownDevices.set(deviceId, webDevice);
                            }
                        }
                        _b.label = 4;
                    case 4:
                        devices_1_1 = devices_1.next();
                        return [3 /*break*/, 2];
                    case 5: return [3 /*break*/, 8];
                    case 6:
                        e_1_1 = _b.sent();
                        e_1 = { error: e_1_1 };
                        return [3 /*break*/, 8];
                    case 7:
                        try {
                            if (devices_1_1 && !devices_1_1.done && (_a = devices_1.return)) _a.call(devices_1);
                        }
                        finally { if (e_1) throw e_1.error; }
                        return [7 /*endfinally*/];
                    case 8: return [2 /*return*/, webDevices];
                }
            });
        });
    };
    WebUSB.prototype.preFilterDevices = function (devices, preFilters) {
        if (!preFilters || !preFilters.length) {
            return devices;
        }
        // Just pre-filter on vid/pid
        return devices.filter(function (device) { return preFilters.some(function (filter) {
            // Vendor
            if (filter.vendorId && filter.vendorId !== device.deviceDescriptor.idVendor)
                return false;
            // Product
            if (filter.productId && filter.productId !== device.deviceDescriptor.idProduct)
                return false;
            // Ignore serial number for node-usb as it requires device connection
            return true;
        }); });
    };
    WebUSB.prototype.filterDevice = function (options, device) {
        if (!options.filters || !options.filters.length) {
            return true;
        }
        return options.filters.some(function (filter) {
            // Vendor
            if (filter.vendorId && filter.vendorId !== device.vendorId)
                return false;
            // Product
            if (filter.productId && filter.productId !== device.productId)
                return false;
            // Class
            if (filter.classCode) {
                if (!device.configuration) {
                    return false;
                }
                // Interface Descriptors
                var match = device.configuration.interfaces.some(function (iface) {
                    // Class
                    if (filter.classCode && filter.classCode !== iface.alternate.interfaceClass)
                        return false;
                    // Subclass
                    if (filter.subclassCode && filter.subclassCode !== iface.alternate.interfaceSubclass)
                        return false;
                    // Protocol
                    if (filter.protocolCode && filter.protocolCode !== iface.alternate.interfaceProtocol)
                        return false;
                    return true;
                });
                if (match) {
                    return true;
                }
            }
            // Class
            if (filter.classCode && filter.classCode !== device.deviceClass)
                return false;
            // Subclass
            if (filter.subclassCode && filter.subclassCode !== device.deviceSubclass)
                return false;
            // Protocol
            if (filter.protocolCode && filter.protocolCode !== device.deviceProtocol)
                return false;
            // Serial
            if (filter.serialNumber && filter.serialNumber !== device.serialNumber)
                return false;
            return true;
        });
    };
    WebUSB.prototype.getDeviceId = function (device) {
        if (device.busNumber === undefined || device.deviceAddress === undefined) {
            return undefined;
        }
        return device.busNumber + "." + device.deviceAddress;
    };
    WebUSB.prototype.isAllowedDevice = function (device) {
        if (this.options.allowAllDevices) {
            return true;
        }
        var isSameDevice = function (device1, device2) {
            return (device1.productId === device2.productId
                && device1.vendorId === device2.vendorId
                && device1.serialNumber === device2.serialNumber);
        };
        for (var i in this.allowedDevices) {
            if (isSameDevice(device, this.allowedDevices[i])) {
                return true;
            }
        }
        return false;
    };
    return WebUSB;
}());
exports.WebUSB = WebUSB;
//# sourceMappingURL=index.js.map