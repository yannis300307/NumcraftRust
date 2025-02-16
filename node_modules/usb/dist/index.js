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
exports.WebUSBDevice = exports.WebUSB = exports.webusb = exports.getWebUsb = exports.findBySerialNumber = exports.findByIds = exports.getDeviceList = exports.useUsbDkBackend = exports.usb = void 0;
var util_1 = require("util");
var webusb_1 = require("./webusb");
Object.defineProperty(exports, "WebUSB", { enumerable: true, get: function () { return webusb_1.WebUSB; } });
Object.defineProperty(exports, "getWebUsb", { enumerable: true, get: function () { return webusb_1.getWebUsb; } });
var webusb_device_1 = require("./webusb/webusb-device");
Object.defineProperty(exports, "WebUSBDevice", { enumerable: true, get: function () { return webusb_device_1.WebUSBDevice; } });
var usb = require("./usb");
exports.usb = usb;
var webusb = new webusb_1.WebUSB();
exports.webusb = webusb;
var getDeviceList = usb.getDeviceList;
exports.getDeviceList = getDeviceList;
var useUsbDkBackend = usb.useUsbDkBackend;
exports.useUsbDkBackend = useUsbDkBackend;
/**
 * Convenience method to get the first device with the specified VID and PID, or `undefined` if no such device is present.
 * @param vid
 * @param pid
 */
var findByIds = function (vid, pid) {
    var devices = usb.getDeviceList();
    return devices.find(function (item) { return item.deviceDescriptor.idVendor === vid && item.deviceDescriptor.idProduct === pid; });
};
exports.findByIds = findByIds;
/**
 * Convenience method to get the device with the specified serial number, or `undefined` if no such device is present.
 * @param serialNumber
 */
var findBySerialNumber = function (serialNumber) { return __awaiter(void 0, void 0, void 0, function () {
    var devices, opened, devices_1, devices_1_1, device, getStringDescriptor, buffer, _a, e_1_1;
    var e_1, _b;
    return __generator(this, function (_c) {
        switch (_c.label) {
            case 0:
                devices = usb.getDeviceList();
                opened = function (device) { return !!device.interfaces; };
                _c.label = 1;
            case 1:
                _c.trys.push([1, 9, 10, 11]);
                devices_1 = __values(devices), devices_1_1 = devices_1.next();
                _c.label = 2;
            case 2:
                if (!!devices_1_1.done) return [3 /*break*/, 8];
                device = devices_1_1.value;
                _c.label = 3;
            case 3:
                _c.trys.push([3, 5, 6, 7]);
                if (!opened(device)) {
                    device.open();
                }
                getStringDescriptor = util_1.promisify(device.getStringDescriptor).bind(device);
                return [4 /*yield*/, getStringDescriptor(device.deviceDescriptor.iSerialNumber)];
            case 4:
                buffer = _c.sent();
                if (buffer && buffer.toString() === serialNumber) {
                    return [2 /*return*/, device];
                }
                return [3 /*break*/, 7];
            case 5:
                _a = _c.sent();
                return [3 /*break*/, 7];
            case 6:
                try {
                    if (opened(device)) {
                        device.close();
                    }
                }
                catch (_d) {
                    // Ignore any errors, device may be a system device or inaccessible
                }
                return [7 /*endfinally*/];
            case 7:
                devices_1_1 = devices_1.next();
                return [3 /*break*/, 2];
            case 8: return [3 /*break*/, 11];
            case 9:
                e_1_1 = _c.sent();
                e_1 = { error: e_1_1 };
                return [3 /*break*/, 11];
            case 10:
                try {
                    if (devices_1_1 && !devices_1_1.done && (_b = devices_1.return)) _b.call(devices_1);
                }
                finally { if (e_1) throw e_1.error; }
                return [7 /*endfinally*/];
            case 11: return [2 /*return*/, undefined];
        }
    });
}); };
exports.findBySerialNumber = findBySerialNumber;
//# sourceMappingURL=index.js.map