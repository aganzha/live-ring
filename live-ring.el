;;; live-ring.el --- Live Ring  -*- lexical-binding: t; -*-

;; Copyright (C) 2026 Aleksey Ganzha

;; Author: Aleksey Ganzha <aganzha@yandex.ru>
;; URL: https://github.com/aganzha/live-ring
;; Version: 0.1.0
;; Package-Requires: ((emacs "30.2"))
;; Keywords: convenience, clipboard

;; This file is not part of GNU Emacs.

;; This program is free software; you can redistribute it and/or modify
;; it under the terms of the GNU General Public License as published by
;; the Free Software Foundation, either version 3 of the License, or
;; (at your option) any later version.

;; This program is distributed in the hope that it will be useful,
;; but WITHOUT ANY WARRANTY; without even the implied warranty of
;; MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
;; GNU General Public License for more details.

;; You should have received a copy of the GNU General Public License
;; along with this program.  If not, see <http://www.gnu.org/licenses/>.

;; Keywords: convenience, clipboard


;;; Commentary:
;; Live copy text from Linux desktop clipboard into Emacs kill-ring.
;; Works on GTK builds (Linux: Wayland/X11).

;;; Installing:
;;; (use-package live-ring
;;;   :vc (:url "https://github.com/aganzha/live-ring"))

;;; Code:

(eval-when-compile
  (declare-function vc-git-repository-url nil)
  (declare-function dbus-register-signal nil)
  )

(defvar live-ring-registration
  (dbus-register-signal
   :session
   nil
   "/io/github/aganzha/LiveRing"
   "io.github.aganzha.LiveRing"
   "PasteChanged"
   #'(lambda (paste-text)
       (kill-new paste-text))))

(defun live-ring-setup ()
  "Setup."
  (when (string-match-p "gtk" (emacs-version))
    (let* ((module-name
            (file-name-base load-file-name))
           (soname (replace-regexp-in-string
                    "-"
                    "_"
                    (format "lib%s.so" module-name)))
           (sopath (concat (file-name-directory load-file-name) soname)))
      (unless (file-exists-p sopath)
        (let ((backend (condition-case err
                           (vc-git-repository-url load-file-name)
                         (error
                          (let ((repo-dir
                                 (replace-regexp-in-string "build\\(-[0-9.]+\\)?" "repos"
                                 (file-name-directory load-file-name))))
                            (string-trim
                             (shell-command-to-string
                              (format "git -C %s remote get-url origin"
                                      (shell-quote-argument repo-dir)))))))))
          (let* ((release (concat
                           (string-replace
                            "git@github.com:"
                            "https://github.com/"
                            (string-remove-suffix ".git" backend))
                           "/releases/download/latest/"
                           soname)))
            (url-copy-file release sopath t))))
      (module-load sopath)
      )
    )
  )
(live-ring-setup)

(provide 'live-ring)
;;; live-ring.el ends here
