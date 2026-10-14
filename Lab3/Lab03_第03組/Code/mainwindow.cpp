#include "mainwindow.h"
#include "ui_mainwindow.h"

#include <random>
#include <QShortcut>

MainWindow::MainWindow(QWidget *parent)
    : QMainWindow(parent)
    , ui(new Ui::MainWindow)
{
    ui->setupUi(this);

    // Shortcut
    auto *faster = new QShortcut(QKeySequence(Qt::Key_Right), this);
    connect(faster, &QShortcut::activated, this, [this] {
        ui->speedSlider->triggerAction(QAbstractSlider::SliderSingleStepAdd);
    });
    auto *slower = new QShortcut(QKeySequence(Qt::Key_Left), this);
    connect(slower, &QShortcut::activated, this, [this] {
        ui->speedSlider->triggerAction(QAbstractSlider::SliderSingleStepSub);
    });

    // pin: 7 15 29 37
    for (int pin : {396, 255, 398, 388}) {
        ledPins.push_back(std::make_unique<Gpio>(pin));
    }
    if (ledPins[0]->isTestMode())
        ui->Logger->appendPlainText("# Gpio not found\n"
                                    "# Now in test mode\n");

    // Print usage
    ui->Logger->appendPlainText("# Shortcut\n"
                                "- Box: Shift + num\n"
                                "- LED Shining: l + s\n"
                                "- Switch On: s\n"
                                "- Swtith Off: S\n"
                                "- Running On: r\n"
                                "- Running Off: R\n"
                                "- Change Direction: c\n"
                                "- Faster: Right\n"
                                "- Slower: Left\n"
                                "- QTE button: num\n");

    // Material
    imgOn = QPixmap(":/images/led_on.png");
    imgOff = QPixmap(":/images/led_off.png");
    ledBoxs   = {ui->ledBox1,   ui->ledBox2,   ui->ledBox3,   ui->ledBox4};
    ledImages = {ui->ledImage1, ui->ledImage2, ui->ledImage3, ui->ledImage4};

    // Switch & Running
    switch_timer = new QTimer(this);
    updateSwitchInterval(ui->speedSlider->value());
    connect(switch_timer, &QTimer::timeout, this, &MainWindow::onTimeout);

    // Gaming
    qte_timer = new QTimer(this);
    updateQteInterval(ui->qteSlider->value());
    connect(qte_timer, &QTimer::timeout, this, &MainWindow::onTimeout);
}

MainWindow::~MainWindow()
{
    delete ui;
}

// Tools
void MainWindow::updateLeds() {
    for (int i = 0; i < ledBoxs.size(); i++) {
        bool on = ledMask & (1u << i);
        ledImages[i]->setPixmap(on ? imgOn : imgOff);
        ledBoxs[i]->setChecked(on);
        ledPins[i]->setValue(on);
    }
}

void MainWindow::clearMode() {
    switch_timer->stop();
    qte_timer->stop();
    ledMask = 0b0000;
    updateLeds();
    mode = Mode::None;
}

void MainWindow::updateSwitchInterval(int value) {
    switch_interval = ui->speedSlider->maximum() - value + 5;
    switch_timer->setInterval(switch_interval * 10);
}

void MainWindow::updateQteInterval(int value) {
    qte_interval = ui->qteSlider->maximum() - value + 10;
    qte_timer->setInterval(qte_interval * 10);
}

void MainWindow::onTimeout() {
    switch (mode) {
    case Mode::Switching:
        ledMask ^= 0b1111;
        break;
    case Mode::Running:
        if (reverse) {
            ledMask = (ledMask >> 1 | ledMask << 3) & 0b1111;
        } else {
            ledMask = (ledMask << 1 | ledMask >> 3) & 0b1111;
        }
        break;
    case Mode::Gaming:
        qteRefresh();
        break;
    default:
        ui->Logger->appendPlainText("State management problem");
        break;
    }
    updateLeds();
}

// Shining
void MainWindow::on_shiningButton_clicked()
{
    if (mode != Mode::None)
        return;

    ledMask = 0;
    for (int i = 0; i < ledBoxs.size(); i++) {
        if (ledBoxs[i]->isChecked())
            ledMask |= (1u << i);
    }

    updateLeds();
}

// Switch
void MainWindow::on_switchOnButton_clicked()
{
    if (mode != Mode::None)
        return;

    ledMask = 0b0011;
    mode = Mode::Switching;
    switch_timer->start();
}

void MainWindow::on_switchOffButton_clicked()
{
    if (mode != Mode::Switching)
        return;

    clearMode();
}

// Running
void MainWindow::on_RunningOnButton_clicked()
{
    if (mode != Mode::None)
        return;

    ledMask = 0b1000;
    mode = Mode::Running;
    switch_timer->start();
}

void MainWindow::on_RunningOffButton_clicked()
{
    if (mode != Mode::Running)
        return;

    clearMode();
}

void MainWindow::on_ChangeDirectionButton_clicked()
{
    reverse = !reverse;
}

// Speed change
void MainWindow::on_speedSlider_valueChanged(int value)
{
    updateSwitchInterval(value);
}

// Gaming
void MainWindow::qteRefresh() {
    static std::mt19937 rng(std::random_device{}());
    static std::uniform_int_distribution<int> dist(0, 3);

    int index, randMask;
    do {
        index = dist(rng);
        randMask = 1u << index;
    } while (ledMask == randMask);

    ledMask = randMask;
    updateLeds();
}

void MainWindow::on_qteStart_clicked()
{
    if (mode != Mode::None)
        return;

    score = 0;
    ui->qteScore->display(score);
    mode = Mode::Gaming;
    qteRefresh();
    qte_timer->start();
}

void MainWindow::on_qteRestart_clicked()
{
    if (mode != Mode::None && mode != Mode::Gaming)
        return;

    score = 0;
    ui->qteScore->display(score);
    mode = Mode::Gaming;
    qteRefresh();
    qte_timer->start();
}


void MainWindow::qteClicked(int index) {
    if (mode != Mode::Gaming)
        return;

    // is click right button
    int mask = 1u << index;
    if (ledMask == mask) {
        score += 20;
        if (score >= 100) {
            ui->Logger->appendPlainText("QTE PASS");
            clearMode();
        } else {
            qte_timer->start();
            qteRefresh();
        }
    } else {
        ui->Logger->appendPlainText("QTE FAIL");
        clearMode();
    }

    ui->qteScore->display(score);
}

void MainWindow::on_qteSlider_valueChanged(int value)
{
    updateQteInterval(value);
}

