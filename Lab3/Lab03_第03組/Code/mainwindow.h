#ifndef MAINWINDOW_H
#define MAINWINDOW_H

#include <QMainWindow>

#include <QCheckBox>
#include <QLabel>
#include <QTimer>

#include <memory>
#include <vector>

#include "Gpio.h"

QT_BEGIN_NAMESPACE
namespace Ui {
class MainWindow;
}
QT_END_NAMESPACE

class MainWindow : public QMainWindow
{
    Q_OBJECT

    enum class Mode {
        None,
        Switching,
        Running,
        Gaming,
    };

public:
    explicit MainWindow(QWidget *parent = nullptr);
    ~MainWindow() override;

private slots:
    void on_shiningButton_clicked();

    void on_switchOnButton_clicked();

    void on_switchOffButton_clicked();

    void on_RunningOnButton_clicked();

    void on_RunningOffButton_clicked();

    void on_ChangeDirectionButton_clicked();

    void on_speedSlider_valueChanged(int value);

    void on_qteStart_clicked();

    void on_qteLed1_clicked() {
        qteClicked(0);
    }

    void on_qteLed2_clicked() {
        qteClicked(1);
    }

    void on_qteLed3_clicked() {
        qteClicked(2);
    }

    void on_qteLed4_clicked() {
        qteClicked(3);
    }

    void on_qteRestart_clicked();

    void on_qteSlider_valueChanged(int value);

private:
    Ui::MainWindow *ui;

    Mode mode = Mode::None;

    std::vector<std::unique_ptr<Gpio>> ledPins;
    QPixmap imgOn, imgOff;
    QList<QCheckBox*> ledBoxs;
    QList<QLabel*> ledImages;
    quint8 ledMask = 0;

    // Switch & Shining
    QTimer *switch_timer;
    bool reverse = false;
    int switch_interval;

    // Gaming
    QTimer *qte_timer;
    int qte_interval;
    int score = 0;

private:
    // Tools
    void updateLeds();
    void clearMode();
    void updateSwitchInterval(int value);
    void updateQteInterval(int value);

    // Switch & Running & Gaming
    void onTimeout();

    // Gaming
    void qteRefresh();
    void qteClicked(int index);
};
#endif // MAINWINDOW_H
